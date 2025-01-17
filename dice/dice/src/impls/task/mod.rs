/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::any::Any;
use std::future::Future;
use std::sync::Arc;

use futures::FutureExt;
use more_futures::cancellation::CancellationContext;
use more_futures::spawner::Spawner;

use crate::impls::task::dice::DiceTask;
use crate::impls::task::dice::DiceTaskInternal;
use crate::impls::task::handle::DiceTaskHandle;

pub(crate) mod dice;
pub(crate) mod handle;
pub(crate) mod promise;
mod state;

#[cfg(test)]
mod tests;

pub(crate) fn spawn_dice_task<S, F>(
    spawner: &dyn Spawner<S>,
    ctx: &S,
    f: impl FnOnce(DiceTaskHandle) -> F,
) -> DiceTask
where
    F: Future<Output = Box<dyn Any + Send>> + Send + 'static,
{
    let internal = DiceTaskInternal::new();
    let handle = DiceTaskHandle {
        internal: internal.clone(),
        cancellations: CancellationContext::todo(),
    };

    let spawned = spawner.spawn(ctx, f(handle).boxed());

    DiceTask {
        internal,
        spawned: Some(spawned),
    }
}

/// Unsafe as this creates a Task that must be completed explicitly otherwise polling will never
/// complete.
pub(crate) unsafe fn sync_dice_task() -> DiceTask {
    let internal = DiceTaskInternal::new();

    DiceTask {
        internal,
        spawned: None,
    }
}
