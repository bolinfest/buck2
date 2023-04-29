"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[8962],{3905:(e,t,n)=>{n.r(t),n.d(t,{MDXContext:()=>i,MDXProvider:()=>g,mdx:()=>_,useMDXComponents:()=>p,withMDXComponents:()=>c});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(){return o=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var r in n)Object.prototype.hasOwnProperty.call(n,r)&&(e[r]=n[r])}return e},o.apply(this,arguments)}function u(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?u(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):u(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function d(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var i=r.createContext({}),c=function(e){return function(t){var n=p(t.components);return r.createElement(e,o({},t,{components:n}))}},p=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},g=function(e){var t=p(e.components);return r.createElement(i.Provider,{value:t},e.children)},s="mdxType",f={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,u=e.parentName,i=d(e,["components","mdxType","originalType","parentName"]),c=p(n),g=a,s=c["".concat(u,".").concat(g)]||c[g]||f[g]||o;return n?r.createElement(s,l(l({ref:t},i),{},{components:n})):r.createElement(s,l({ref:t},i))}));function _(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,u=new Array(o);u[0]=m;var l={};for(var d in t)hasOwnProperty.call(t,d)&&(l[d]=t[d]);l.originalType=e,l[s]="string"==typeof e?e:a,u[1]=l;for(var i=2;i<o;i++)u[i]=n[i];return r.createElement.apply(null,u)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},71669:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>d,contentTitle:()=>u,default:()=>p,frontMatter:()=>o,metadata:()=>l,toc:()=>i});var r=n(87462),a=(n(67294),n(3905));const o={id:"unconfigured_target_node"},u="unconfigured_target_node type",l={unversionedId:"api/bxl/unconfigured_target_node",id:"api/bxl/unconfigured_target_node",title:"unconfigured_target_node type",description:"Methods for unconfigured target node.",source:"@site/../docs/api/bxl/unconfigured_target_node.generated.md",sourceDirName:"api/bxl",slug:"/api/bxl/unconfigured_target_node",permalink:"/docs/api/bxl/unconfigured_target_node",draft:!1,tags:[],version:"current",frontMatter:{id:"unconfigured_target_node"},sidebar:"manualSidebar",previous:{title:"target_node type",permalink:"/docs/api/bxl/target_node"},next:{title:"uqueryctx type",permalink:"/docs/api/bxl/uqueryctx"}},d={},i=[{value:"unconfigured_target_node.attributes",id:"unconfigured_target_nodeattributes",level:2},{value:"unconfigured_target_node.attrs",id:"unconfigured_target_nodeattrs",level:2},{value:"unconfigured_target_node.label",id:"unconfigured_target_nodelabel",level:2}],c={toc:i};function p(e){let{components:t,...n}=e;return(0,a.mdx)("wrapper",(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.mdx)("h1",{id:"unconfigured_target_node-type"},(0,a.mdx)("inlineCode",{parentName:"h1"},"unconfigured_target_node")," type"),(0,a.mdx)("p",null,"Methods for unconfigured target node."),(0,a.mdx)("h2",{id:"unconfigured_target_nodeattributes"},"unconfigured","_","target","_","node.attributes"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-python"},'unconfigured_target_node.attributes: ""\n')),(0,a.mdx)("p",null,"DO NOT USE. Will be deprecated."),(0,a.mdx)("p",null,"Gets the coerced attributes from the unconfigured target node. Returns an iterable ",(0,a.mdx)("inlineCode",{parentName:"p"},"starlark_attributes"),"\nobject."),(0,a.mdx)("p",null,"Sample usage:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-text"},"def _impl_attributes(ctx):\n    target_node = ctx.uquery().eval(\"owner('path/to/file')\")[0]\n    for attr in target_node.attributes:\n        ctx.output.print(attr)\n")),(0,a.mdx)("hr",null),(0,a.mdx)("h2",{id:"unconfigured_target_nodeattrs"},"unconfigured","_","target","_","node.attrs"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-python"},'unconfigured_target_node.attrs: ""\n')),(0,a.mdx)("p",null,"Gets the coerced attributes from the unconfigured target node. Returns a struct."),(0,a.mdx)("p",null,"Sample usage:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-text"},"def _impl_attributes(ctx):\n    target_node = ctx.uquery().eval(\"owner('path/to/file')\")[0]\n    ctx.output.print(target_node.attrs.my_attr)\n")),(0,a.mdx)("hr",null),(0,a.mdx)("h2",{id:"unconfigured_target_nodelabel"},"unconfigured","_","target","_","node.label"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-python"},'unconfigured_target_node.label: "target_label"\n')),(0,a.mdx)("p",null,"Gets the label from the unconfigured target node."),(0,a.mdx)("p",null,"Sample usage:"),(0,a.mdx)("pre",null,(0,a.mdx)("code",{parentName:"pre",className:"language-text"},"def _impl_label(ctx):\n    target_node = ctx.uquery().eval(\"owner('path/to/file')\")[0]\n    ctx.output.print(target_node.label)\n")))}p.isMDXComponent=!0}}]);