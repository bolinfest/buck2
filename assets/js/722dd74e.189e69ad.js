"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9236],{3905:(e,r,t)=>{t.r(r),t.d(r,{MDXContext:()=>s,MDXProvider:()=>u,mdx:()=>v,useMDXComponents:()=>c,withMDXComponents:()=>m});var n=t(67294);function o(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function i(){return i=Object.assign||function(e){for(var r=1;r<arguments.length;r++){var t=arguments[r];for(var n in t)Object.prototype.hasOwnProperty.call(t,n)&&(e[n]=t[n])}return e},i.apply(this,arguments)}function p(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);r&&(n=n.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,n)}return t}function a(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?p(Object(t),!0).forEach((function(r){o(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):p(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function l(e,r){if(null==e)return{};var t,n,o=function(e,r){if(null==e)return{};var t,n,o={},i=Object.keys(e);for(n=0;n<i.length;n++)t=i[n],r.indexOf(t)>=0||(o[t]=e[t]);return o}(e,r);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)t=i[n],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(o[t]=e[t])}return o}var s=n.createContext({}),m=function(e){return function(r){var t=c(r.components);return n.createElement(e,i({},r,{components:t}))}},c=function(e){var r=n.useContext(s),t=r;return e&&(t="function"==typeof e?e(r):a(a({},r),e)),t},u=function(e){var r=c(e.components);return n.createElement(s.Provider,{value:r},e.children)},d="mdxType",f={inlineCode:"code",wrapper:function(e){var r=e.children;return n.createElement(n.Fragment,{},r)}},y=n.forwardRef((function(e,r){var t=e.components,o=e.mdxType,i=e.originalType,p=e.parentName,s=l(e,["components","mdxType","originalType","parentName"]),m=c(t),u=o,d=m["".concat(p,".").concat(u)]||m[u]||f[u]||i;return t?n.createElement(d,a(a({ref:r},s),{},{components:t})):n.createElement(d,a({ref:r},s))}));function v(e,r){var t=arguments,o=r&&r.mdxType;if("string"==typeof e||o){var i=t.length,p=new Array(i);p[0]=y;var a={};for(var l in r)hasOwnProperty.call(r,l)&&(a[l]=r[l]);a.originalType=e,a[d]="string"==typeof e?e:o,p[1]=a;for(var s=2;s<i;s++)p[s]=t[s];return n.createElement.apply(null,p)}return n.createElement.apply(null,t)}y.displayName="MDXCreateElement"},10249:(e,r,t)=>{t.r(r),t.d(r,{assets:()=>l,contentTitle:()=>p,default:()=>c,frontMatter:()=>i,metadata:()=>a,toc:()=>s});var n=t(87462),o=(t(67294),t(3905));const i={id:"promise"},p="promise type",a={unversionedId:"api/build/promise",id:"api/build/promise",title:"promise type",description:"promise.join",source:"@site/../docs/api/build/promise.generated.md",sourceDirName:"api/build",slug:"/api/build/promise",permalink:"/docs/api/build/promise",draft:!1,tags:[],version:"current",frontMatter:{id:"promise"},sidebar:"manualSidebar",previous:{title:"label_relative_path type",permalink:"/docs/api/build/label_relative_path"},next:{title:"action type",permalink:"/docs/api/bxl/action"}},l={},s=[{value:"promise.join",id:"promisejoin",level:2},{value:"promise.map",id:"promisemap",level:2}],m={toc:s};function c(e){let{components:r,...t}=e;return(0,o.mdx)("wrapper",(0,n.Z)({},m,t,{components:r,mdxType:"MDXLayout"}),(0,o.mdx)("h1",{id:"promise-type"},(0,o.mdx)("inlineCode",{parentName:"h1"},"promise")," type"),(0,o.mdx)("h2",{id:"promisejoin"},"promise.join"),(0,o.mdx)("pre",null,(0,o.mdx)("code",{parentName:"pre",className:"language-python"},'def promise.join(*args) -> "promise"\n')),(0,o.mdx)("p",null,"Join a set of promises together into a single promise."),(0,o.mdx)("p",null,"Given a series of promises, ",(0,o.mdx)("inlineCode",{parentName:"p"},"p4 = p1.join(p2, p3)")," will produce a promise\nwhere the value is promise that resolves to a tuple containing the three values,\nthose from ",(0,o.mdx)("inlineCode",{parentName:"p"},"p1"),", ",(0,o.mdx)("inlineCode",{parentName:"p"},"p2")," and ",(0,o.mdx)("inlineCode",{parentName:"p"},"p3")," respectively."),(0,o.mdx)("hr",null),(0,o.mdx)("h2",{id:"promisemap"},"promise.map"),(0,o.mdx)("pre",null,(0,o.mdx)("code",{parentName:"pre",className:"language-python"},'def promise.map(func: "") -> "promise"\n')),(0,o.mdx)("p",null,"Given a promise, apply a function to the value it contains, producing a promise with the resulting value."))}c.isMDXComponent=!0}}]);