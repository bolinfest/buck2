/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::collections::HashMap;
use std::collections::HashSet;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use buck2_core::env_helper::EnvHelper;
use buck2_core::target::label::ConfiguredTargetLabel;
use buck2_critical_path::compute_critical_path_potentials;
use buck2_critical_path::GraphBuilder;
use buck2_critical_path::OptionalVertexId;
use buck2_data::ToProtoMessage;
use buck2_events::dispatch::instant_event;
use buck2_events::dispatch::with_dispatcher_async;
use buck2_events::dispatch::EventDispatcher;
use buck2_events::metadata;
use buck2_events::span::SpanId;
use buck2_node::nodes::configured::ConfiguredTargetNode;
use derive_more::Display;
use derive_more::From;
use dice::UserComputationData;
use dupe::Dupe;
use dupe::OptionDupedExt;
use gazebo::prelude::VecExt;
use itertools::Itertools;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

use crate::actions::key::ActionKey;
use crate::actions::RegisteredAction;
use crate::artifact_groups::ArtifactGroup;
use crate::artifact_groups::TransitiveSetProjectionKey;
use crate::deferred::base_deferred_key::BaseDeferredKey;

pub struct ActionExecutionSignal {
    pub action: Arc<RegisteredAction>,
    pub duration: NodeDuration,
    pub span_id: Option<SpanId>,
}

pub struct AnalysisSignal {
    pub label: ConfiguredTargetLabel,
    pub node: ConfiguredTargetNode,
    pub duration: NodeDuration,
    pub span_id: Option<SpanId>,
}

#[derive(Copy, Clone, Dupe)]
pub struct NodeDuration {
    /// The amount of time for this node that corresponds to something the user might be able to
    /// improve. We should better break this down.
    pub user: Duration,
    /// The total duration for this node.
    pub total: Duration,
}

impl NodeDuration {
    /// Returns the duration we are using in our critical path calculation. This doesn't really
    /// *need* to be a function but right now we use user and want to switch to total so it's
    /// eaiser to do that if this is in a single function.
    fn critical_path_duration(&self) -> Duration {
        self.user
    }

    fn zero() -> Self {
        Self {
            user: Duration::from_secs(0),
            total: Duration::from_secs(0),
        }
    }
}
pub struct TransitiveSetComputationSignal {
    pub key: TransitiveSetProjectionKey,
    pub artifacts: HashSet<ActionKey>,
    pub set_deps: HashSet<TransitiveSetProjectionKey>,
}

/// When dealing with dynamic outputs, we obtain an action key that'll result in us doing
/// (deferred) analysis and resolve to another action (the deferred action). We then go and execute
/// that action instead. Our predecessor map tracks action keys so we need to track this
/// redirection otherwise we'll have a broken link in our chain.
pub struct ActionRedirectionSignal {
    pub key: ActionKey,
    pub dest: ActionKey,
}

pub struct TopLevelTargetSignal {
    pub label: ConfiguredTargetLabel,
    pub artifacts: Vec<ArtifactGroup>,
}

/* These signals are distinct from the main Buck event bus because some
 * analysis needs access to the entire build graph, and serializing the
 * entire build graph isn't feasible - therefore, we have these signals
 * with an unserializable but lightweight handle on a RegisteredAction.
 */
#[derive(From)]
pub enum BuildSignal {
    ActionExecution(ActionExecutionSignal),
    TransitiveSetComputation(TransitiveSetComputationSignal),
    ActionRedirection(ActionRedirectionSignal),
    Analysis(AnalysisSignal),
    TopLevelTarget(TopLevelTargetSignal),
    BuildFinished,
}

#[derive(Clone, Dupe)]
pub struct BuildSignalSender {
    sender: Arc<UnboundedSender<BuildSignal>>,
}

impl BuildSignalSender {
    pub fn signal(&self, signal: impl Into<BuildSignal>) {
        let _ignore_error = self.sender.send(signal.into());
    }
}

#[derive(Clone, Dupe)]
struct CriticalPathNode<TKey: Eq, TValue> {
    /// The aggregated duration of this critical path.
    pub duration: Duration,
    /// The value of this node. If None, this node just won't be included when displaying.
    pub value: TValue,
    pub prev: Option<TKey>,
}

#[derive(Hash, Eq, PartialEq, Clone, Dupe, Debug, Display)]
pub enum NodeKey {
    ActionKey(ActionKey),
    TransitiveSetProjection(TransitiveSetProjectionKey),
    // NOTE: we do not currently support analysis of anonymous targets or BXL.
    Analysis(ConfiguredTargetLabel),
}

pub struct BuildSignalReceiver<T> {
    receiver: UnboundedReceiverStream<BuildSignal>,
    backend: T,
}

fn extract_critical_path<TKey: Hash + Eq, TValue>(
    predecessors: &HashMap<TKey, CriticalPathNode<TKey, TValue>>,
) -> Vec<(&TKey, &TValue, Duration)> {
    let terminal = predecessors
        .iter()
        .max_by_key(|(_key, data)| data.duration)
        .map(|q| q.0);
    let mut path = itertools::unfold(terminal, |maybe_key| {
        if maybe_key.is_none() {
            return None;
        }
        let key = maybe_key.unwrap();
        let next = predecessors.get(key);
        *maybe_key = next.and_then(|q| (q.prev).as_ref());
        next.map(|x| (key, &x.value, x.duration))
    })
    .collect::<Vec<_>>();
    // Take differences of adjacent elements to recover action time from cumulative sum.
    path.reverse();
    for i in (1..path.len()).rev() {
        path[i].2 = path[i].2.saturating_sub(path[i - 1].2);
    }

    path
}

impl<T> BuildSignalReceiver<T>
where
    T: BuildListenerBackend,
{
    fn new(receiver: UnboundedReceiver<BuildSignal>, backend: T) -> Self {
        Self {
            receiver: UnboundedReceiverStream::new(receiver),
            backend,
        }
    }

    pub async fn run_and_log(mut self) -> anyhow::Result<()> {
        while let Some(event) = self.receiver.next().await {
            match event {
                BuildSignal::ActionExecution(execution) => self.process_action(execution)?,
                BuildSignal::TransitiveSetComputation(tset) => {
                    self.process_transitive_set_computation(tset)?
                }
                BuildSignal::ActionRedirection(redirection) => {
                    self.process_action_redirection(redirection)?
                }
                BuildSignal::Analysis(analysis) => self.process_analysis(analysis)?,
                BuildSignal::TopLevelTarget(top_level) => {
                    self.process_top_level_target(top_level)?
                }
                BuildSignal::BuildFinished => break,
            }
        }

        let BuildInfo {
            critical_path,
            num_nodes,
            num_edges,
        } = self.backend.finish()?;

        let critical_path2 = critical_path
            .iter()
            .filter_map(|(key, data, potential_improvement)| {
                let entry: buck2_data::critical_path_entry2::Entry = match key {
                    NodeKey::ActionKey(action_key) => {
                        let owner = match action_key.owner() {
                            BaseDeferredKey::TargetLabel(t) => t.as_proto().into(),
                            BaseDeferredKey::AnonTarget(t) => t.as_proto().into(),
                            BaseDeferredKey::BxlLabel(t) => t.as_proto().into(),
                        };

                        // If we have a NodeKey that's an ActionKey we'd expect to have an `action`
                        // in our data.
                        let action = data.action.as_ref()?;

                        buck2_data::critical_path_entry2::ActionExecution {
                            owner: Some(owner),
                            name: Some(buck2_data::ActionName {
                                category: action.category().as_str().to_owned(),
                                identifier: action.identifier().unwrap_or("").to_owned(),
                            }),
                        }
                        .into()
                    }
                    NodeKey::Analysis(key) => buck2_data::critical_path_entry2::Analysis {
                        target: Some(key.as_proto().into()),
                    }
                    .into(),
                    NodeKey::TransitiveSetProjection(..) => return None,
                };

                Some((entry, data, potential_improvement))
            })
            .map(|(entry, data, potential_improvement)| {
                anyhow::Ok(buck2_data::CriticalPathEntry2 {
                    span_id: data.span_id.map(|span_id| span_id.into()),
                    duration: Some(data.duration.critical_path_duration().try_into()?),
                    user_duration: Some(data.duration.user.try_into()?),
                    total_duration: Some(data.duration.total.try_into()?),
                    potential_improvement_duration: potential_improvement
                        .map(|p| p.try_into())
                        .transpose()?,
                    entry: Some(entry),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        instant_event(buck2_data::BuildGraphExecutionInfo {
            critical_path: Vec::new(),
            critical_path2,
            metadata: metadata::collect(),
            num_nodes,
            num_edges,
            uses_total_duration: false,
        });
        Ok(())
    }

    fn process_action(&mut self, execution: ActionExecutionSignal) -> Result<(), anyhow::Error> {
        // Identify most costly predecessor.
        let inputs = execution.action.inputs()?;

        let dep_keys = inputs
            .iter()
            .filter_map(|dep| match dep {
                ArtifactGroup::Artifact(artifact) => {
                    artifact.action_key().duped().map(NodeKey::ActionKey)
                }
                ArtifactGroup::TransitiveSetProjection(key) => {
                    Some(NodeKey::TransitiveSetProjection(key.dupe()))
                }
            })
            .chain(
                execution
                    .action
                    .owner()
                    .unpack_target_label()
                    .duped()
                    .map(NodeKey::Analysis)
                    .into_iter(),
            );

        self.backend.process_node(
            NodeKey::ActionKey(execution.action.key().dupe()),
            Some(execution.action.dupe()),
            execution.duration,
            dep_keys,
            execution.span_id,
        );

        Ok(())
    }

    fn process_action_redirection(
        &mut self,
        redirection: ActionRedirectionSignal,
    ) -> anyhow::Result<()> {
        self.backend.process_node(
            NodeKey::ActionKey(redirection.key),
            None,
            NodeDuration::zero(), // Those nodes don't carry a duration.
            std::iter::once(NodeKey::ActionKey(redirection.dest)),
            None,
        );

        Ok(())
    }

    fn process_transitive_set_computation(
        &mut self,
        set: TransitiveSetComputationSignal,
    ) -> anyhow::Result<()> {
        let artifacts = set.artifacts.into_iter().map(NodeKey::ActionKey);
        let sets = set
            .set_deps
            .into_iter()
            .map(NodeKey::TransitiveSetProjection);

        self.backend.process_node(
            NodeKey::TransitiveSetProjection(set.key),
            None,
            NodeDuration::zero(), // Those nodes don't carry a duration.
            artifacts.chain(sets),
            None,
        );

        Ok(())
    }

    fn process_analysis(&mut self, analysis: AnalysisSignal) -> Result<(), anyhow::Error> {
        let dep_keys = analysis
            .node
            .deps()
            .map(|d| NodeKey::Analysis(d.label().dupe()));

        self.backend.process_node(
            NodeKey::Analysis(analysis.label),
            None,
            analysis.duration,
            dep_keys,
            analysis.span_id,
        );

        Ok(())
    }

    // TODO: We would need something similar with anon targets.
    fn process_top_level_target(
        &mut self,
        top_level: TopLevelTargetSignal,
    ) -> Result<(), anyhow::Error> {
        let artifact_keys = top_level.artifacts.into_iter().filter_map(|dep| match dep {
            ArtifactGroup::Artifact(artifact) => {
                artifact.action_key().duped().map(NodeKey::ActionKey)
            }
            ArtifactGroup::TransitiveSetProjection(key) => {
                Some(NodeKey::TransitiveSetProjection(key.dupe()))
            }
        });

        self.backend
            .process_top_level_target(NodeKey::Analysis(top_level.label), artifact_keys);

        Ok(())
    }
}

pub trait BuildListenerBackend {
    fn process_node(
        &mut self,
        key: NodeKey,
        value: Option<Arc<RegisteredAction>>,
        duration: NodeDuration,
        dep_keys: impl Iterator<Item = NodeKey>,
        span_id: Option<SpanId>,
    );

    fn process_top_level_target(
        &mut self,
        analysis: NodeKey,
        artifacts: impl Iterator<Item = NodeKey>,
    );

    fn finish(self) -> anyhow::Result<BuildInfo>;
}

pub struct BuildInfo {
    // Node, its data, and its potential for improvement
    critical_path: Vec<(NodeKey, NodeData, Option<Duration>)>,
    num_nodes: u64,
    num_edges: u64,
}

struct DefaultBackend {
    predecessors: HashMap<NodeKey, CriticalPathNode<NodeKey, NodeData>>,
    num_nodes: u64,
    num_edges: u64,
}

impl DefaultBackend {
    fn new() -> Self {
        Self {
            predecessors: HashMap::new(),
            num_nodes: 0,
            num_edges: 0,
        }
    }
}

impl BuildListenerBackend for DefaultBackend {
    fn process_node(
        &mut self,
        key: NodeKey,
        value: Option<Arc<RegisteredAction>>,
        duration: NodeDuration,
        dep_keys: impl Iterator<Item = NodeKey>,
        span_id: Option<SpanId>,
    ) {
        let longest_ancestor = dep_keys
            .unique()
            .filter_map(|node_key| {
                self.num_edges += 1;
                let node_data = self.predecessors.get(&node_key)?;
                Some((node_key, node_data.duration))
            })
            .max_by_key(|d| d.1);

        let value = NodeData {
            action: value,
            duration,
            span_id,
        };

        let node = match longest_ancestor {
            Some((key, ancestor_duration)) => CriticalPathNode {
                prev: Some(key.dupe()),
                value,
                duration: ancestor_duration + duration.critical_path_duration(),
            },
            None => CriticalPathNode {
                prev: None,
                value,
                duration: duration.critical_path_duration(),
            },
        };

        self.num_nodes += 1;
        self.predecessors.insert(key, node);
    }

    fn process_top_level_target(
        &mut self,
        _analyis: NodeKey,
        _artifacts: impl Iterator<Item = NodeKey>,
    ) {
    }

    fn finish(self) -> anyhow::Result<BuildInfo> {
        let critical_path = extract_critical_path(&self.predecessors)
            .into_map(|(key, data, _duration)| (key.dupe(), data.dupe(), None));

        Ok(BuildInfo {
            critical_path,
            num_nodes: self.num_nodes,
            num_edges: self.num_edges,
        })
    }
}

/// An implementation of critical path that uses a longest-paths graph in order to produce
/// potential savings in addition to the critical path.
struct LongestPathGraphBackend {
    builder: anyhow::Result<GraphBuilder<NodeKey, NodeData>>,
    top_level_analysis: Vec<VisibilityEdge>,
}

#[derive(Dupe, Clone)]
struct NodeData {
    action: Option<Arc<RegisteredAction>>,
    duration: NodeDuration,
    span_id: Option<SpanId>,
}

/// Represents nodes that block us "seeing" other parts of the graph until they finish evaluating.
struct VisibilityEdge {
    node: NodeKey,
    makes_visible: Vec<NodeKey>,
}

impl LongestPathGraphBackend {
    fn new() -> Self {
        Self {
            builder: Ok(GraphBuilder::new()),
            top_level_analysis: Vec::new(),
        }
    }
}

impl BuildListenerBackend for LongestPathGraphBackend {
    fn process_node(
        &mut self,
        key: NodeKey,
        action: Option<Arc<RegisteredAction>>,
        duration: NodeDuration,
        dep_keys: impl Iterator<Item = NodeKey>,
        span_id: Option<SpanId>,
    ) {
        let builder = match self.builder.as_mut() {
            Ok(b) => b,
            Err(..) => return,
        };

        let res = builder.push(
            key,
            dep_keys,
            NodeData {
                action,
                duration,
                span_id,
            },
        );

        match res {
            Ok(()) => {}
            Err(e) => self.builder = Err(e.into()),
        }
    }

    fn process_top_level_target(
        &mut self,
        analysis: NodeKey,
        artifacts: impl Iterator<Item = NodeKey>,
    ) {
        self.top_level_analysis.push(VisibilityEdge {
            node: analysis,
            makes_visible: artifacts.collect(),
        })
    }

    fn finish(self) -> anyhow::Result<BuildInfo> {
        let (graph, keys, mut data) = self.builder?.finish();

        let mut first_analysis = graph.allocate_vertex_data(OptionalVertexId::none());

        for visibility in &self.top_level_analysis {
            let analysis = &visibility.node;
            let artifacts = &visibility.makes_visible;

            let analysis = match keys.get(analysis) {
                Some(k) => k,
                None => continue, // Nothing depends on this,
            };

            let mut queue = Vec::new();

            for artifact in artifacts {
                let artifact = match keys.get(artifact) {
                    Some(a) => a,
                    None => {
                        // Not built. Unexpected, but we don't report signals in all failure cases so that can happen.
                        continue;
                    }
                };

                queue.push(artifact);

                while let Some(i) = queue.pop() {
                    if first_analysis[i].is_some() {
                        continue;
                    }

                    // Only add those new edges on things that analysis cannot depend on.
                    match keys[i] {
                        NodeKey::Analysis(..) => continue,
                        NodeKey::ActionKey(..) | NodeKey::TransitiveSetProjection(..) => {}
                    };

                    first_analysis[i] = analysis.into();
                    queue.extend(graph.iter_edges(i));
                }
            }
        }

        let graph = graph
            .add_edges(&first_analysis)
            .context("Error adding first_analysis edges to graph")?;

        let durations = data.try_map_ref(|d| {
            d.duration
                .critical_path_duration()
                .as_micros()
                .try_into()
                .context("Duration `as_micros()` exceeds u64")
        })?;

        let (critical_path, critical_path_cost, replacement_durations) =
            compute_critical_path_potentials(&graph, &durations)
                .context("Error computing critical path potentials")?;

        drop(durations);

        let critical_path = critical_path
            .iter()
            .map(|(cp_idx, vertex_idx)| {
                let vertex_idx = *vertex_idx;
                let key = keys[vertex_idx].dupe();

                // OK to replace `data` with empty things here because we know that we will not access
                // the same index twice.
                let data = std::mem::replace(
                    &mut data[vertex_idx],
                    NodeData {
                        action: None,
                        duration: NodeDuration::zero(),
                        span_id: None,
                    },
                );

                let potential = critical_path_cost - replacement_durations[cp_idx];

                (key, data, Some(Duration::from_micros(potential.runtime)))
            })
            .collect();

        Ok(BuildInfo {
            critical_path,
            num_nodes: graph.vertices_count() as _,
            num_edges: graph.edges_count() as _,
        })
    }
}

pub trait SetBuildSignals {
    fn set_build_signals(&mut self, sender: BuildSignalSender);
}

impl SetBuildSignals for UserComputationData {
    fn set_build_signals(&mut self, sender: BuildSignalSender) {
        self.data.set(sender);
    }
}

pub trait HasBuildSignals {
    fn get_build_signals(&self) -> Option<&BuildSignalSender>;
}

impl HasBuildSignals for UserComputationData {
    fn get_build_signals(&self) -> Option<&BuildSignalSender> {
        self.data.get::<BuildSignalSender>().ok()
    }
}

fn start_listener(
    events: EventDispatcher,
    backend: impl BuildListenerBackend + Send + 'static,
) -> (BuildSignalSender, JoinHandle<anyhow::Result<()>>) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    let sender = BuildSignalSender {
        sender: Arc::new(sender),
    };

    let listener = BuildSignalReceiver::new(receiver, backend);
    let receiver_task_handle = tokio::spawn(with_dispatcher_async(events.dupe(), async move {
        listener.run_and_log().await
    }));

    (sender, receiver_task_handle)
}

/// Creates a Build Listener signal pair and invokes the given asynchronous function with the send-end of the signal
/// sender.
///
/// Build listeners in this module operate by creating a matched pair of signal senders and signal receivers. Senders
/// are Dupe and allow for arbitrarily many writeres. Receivers are not Dupe and are expected to be driven by a single
/// thread. This implies that, in order for the receiver to function correctly and dispatch to build listeners, it must
/// be run in a backround task that is periodically polled.
///
/// This function arranges for a background task to be spawned that drives the receiver, while invoking the called
/// function with a live BuildSignalSender that can be used to send events to the listening receiver. Upon return of
/// `scope`, the sender terminates the receiver by sending a `BuildFinished` signal and joins the receiver task.
pub async fn scope<F, R, Fut>(events: EventDispatcher, func: F) -> anyhow::Result<R>
where
    F: FnOnce(BuildSignalSender) -> Fut,
    Fut: Future<Output = anyhow::Result<R>>,
{
    static USE_LONGEST_PATH_GRAPH: EnvHelper<bool> = EnvHelper::new("BUCK2_USE_LONGEST_PATH_GRAPH");
    let use_longest_path_graph = USE_LONGEST_PATH_GRAPH.get_copied()?.unwrap_or_default();

    let (sender, handle) = if use_longest_path_graph {
        start_listener(events, LongestPathGraphBackend::new())
    } else {
        start_listener(events, DefaultBackend::new())
    };
    let result = func(sender.dupe()).await;
    sender.signal(BuildSignal::BuildFinished);
    handle.await??;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    type CriticalPathMap = HashMap<i32, CriticalPathNode<i32, Option<i32>>>;

    fn cp_insert(
        predecessors: &mut CriticalPathMap,
        key: i32,
        prev: Option<i32>,
        duration: Duration,
    ) {
        predecessors.insert(
            key,
            CriticalPathNode {
                duration,
                value: Some(key),
                prev,
            },
        );
    }
    #[test]
    fn empty_path() {
        let predecessors = CriticalPathMap::new();
        assert_eq!(extract_critical_path(&predecessors), vec![]);
    }

    #[test]
    fn unit_path() {
        let mut predecessors = CriticalPathMap::new();
        cp_insert(&mut predecessors, 1, None, Duration::from_secs(3));
        assert_eq!(
            extract_critical_path(&predecessors),
            vec![(&1, &Some(1), Duration::from_secs(3))],
        );
    }

    #[test]
    fn long_path() {
        let mut predecessors = HashMap::new();
        /*   -> 1 -> 2 -> 3
         *   5s   6s   7s
         *
         *      1 -> 4
         *        9s
         */
        cp_insert(&mut predecessors, 1, None, Duration::from_secs(5));
        cp_insert(&mut predecessors, 2, Some(1), Duration::from_secs(11));
        cp_insert(&mut predecessors, 3, Some(2), Duration::from_secs(18));
        cp_insert(&mut predecessors, 4, Some(1), Duration::from_secs(14));
        assert_eq!(
            extract_critical_path(&predecessors),
            vec![
                (&1, &Some(1), Duration::from_secs(5)),
                (&2, &Some(2), Duration::from_secs(6)),
                (&3, &Some(3), Duration::from_secs(7)),
            ],
        );
    }
}
