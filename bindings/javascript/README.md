## JavaScript bindings
This generates bindings for the composed module for JavaScript using 
the JavaScript Component tookchain (JCO).

### Generate bindings
```console
$ npm run bindings

> javascript@1.0.0 bindings
> npx jco transpile $npm_package_config_wasm_file -o dist --name composed --map wasi:nn/*=./nn.js#* --map inf:wasi/config-types=./config-types.js


Transpiled JS Component Files:

 - dist/composed.core.wasm                        66.2 KiB
 - dist/composed.core2.wasm                       16.5 KiB
 - dist/composed.core3.wasm                        128 KiB
 - dist/composed.core4.wasm                       16.5 KiB
 - dist/composed.core5.wasm                       1.77 MiB
 - dist/composed.d.ts                             1.06 KiB
 - dist/composed.js                                105 KiB
 - dist/interfaces/inf-wasi-config-types.d.ts      0.1 KiB
 - dist/interfaces/wasi-cli-environment.d.ts      0.09 KiB
 - dist/interfaces/wasi-cli-exit.d.ts             0.16 KiB
 - dist/interfaces/wasi-cli-stderr.d.ts           0.16 KiB
 - dist/interfaces/wasi-cli-stdin.d.ts            0.15 KiB
 - dist/interfaces/wasi-cli-stdout.d.ts           0.16 KiB
 - dist/interfaces/wasi-clocks-wall-clock.d.ts    0.11 KiB
 - dist/interfaces/wasi-filesystem-preopens.d.ts  0.18 KiB
 - dist/interfaces/wasi-filesystem-types.d.ts     2.67 KiB
 - dist/interfaces/wasi-io-error.d.ts             0.08 KiB
 - dist/interfaces/wasi-io-streams.d.ts           0.57 KiB
 - dist/interfaces/wasi-nn-errors.d.ts            0.38 KiB
 - dist/interfaces/wasi-nn-graph.d.ts             0.69 KiB
 - dist/interfaces/wasi-nn-inference.d.ts         0.66 KiB
 - dist/interfaces/wasi-nn-tensor.d.ts  
```
This could then be included in a user application. Currently this does not
work as there is now shim for `wasi:nn` but once that is in place this should
work.


### Development
It can be useful to npm link jco to this project so that changes to jco can be
tested:
```console
$ cd /path/to/jco
$ sudo npm link
$ sudo npm link ./packages/preview2-shim/
```
The from this directory link:
```console
$ npm link @bytecodealliance/jco
$ ln -s /usr/local/lib/node_modules/@bytecodealliance/preview2-shim node_modules/@bytecodealliance/preview2-shim
```
And this would have created the following links:
```console
$ ls -l /usr/local/lib/node_modules/@bytecodealliance/
jco -> ../../../../../home/danielbevenius/work/wasm/jco
preview2-shim -> ../../../../../home/danielbevenius/work/wasm/jco/packages/preview2-shim
```

### shims
I'm currently looking into how to shim the `wasi:nn` namespace. At the moment
if we run `npm run bindings` there are a number of TypeScript definition file
generated for the `wasi:nn` namespace:
```console
dist/interfaces/wasi-nn-errors.d.ts
dist/interfaces/wasi-nn-graph.d.ts
dist/interfaces/wasi-nn-inference.d.ts
dist/interfaces/wasi-nn-tensor.d.ts
```
Now, just keep in mind that these are just the TypeScript definition files and
they only describe types and are not the actual implementation. 

If we look in dist/composed.js we can see the following import:
```js
import { graph, inference } from '@bytecodealliance/preview2-shim/nn';
```
So this is expecting a the @bytecodealliance/preview2-shim package to have an
nn module which exports the graph and inference objects.
We can add this nn.js file to the preview2-shim package and add exports for the
graph and inference objects.

I've also added the following to transpile.js:
```console
$ git diff src/cmd/transpile.js
diff --git a/src/cmd/transpile.js b/src/cmd/transpile.js
index 1cc9a39..8b26021 100644
--- a/src/cmd/transpile.js
+++ b/src/cmd/transpile.js
@@ -116,6 +116,7 @@ export async function transpileComponent (component, opts = {}) {
       'wasi:io/*': '@bytecodealliance/preview2-shim/io#*',
       'wasi:random/*': '@bytecodealliance/preview2-shim/random#*',
       'wasi:sockets/*': '@bytecodealliance/preview2-shim/sockets#*',
+      'wasi:nn/*': '@bytecodealliance/preview2-shim/nn#*',
     }, opts.map || {});
   }
```

With that in place we can call the compute function and see that the inf-wasi
inference component gets called:
```console
$ npm run compute

> javascript@1.0.0 compute
> node app.js

JavaScript inference...
Inference Component Running inference

thread '<unnamed>' panicked at engine/src/engine.rs:9:1:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
wasm://wasm/00080b96:1


RuntimeError: unreachable
    at __rust_start_panic (wasm://wasm/00080b96:wasm-function[172]:0xc938)
    at rust_panic (wasm://wasm/00080b96:wasm-function[164]:0xc743)
    at _ZN3std9panicking20rust_panic_with_hook17h9c783872fdb901ccE (wasm://wasm/00080b96:wasm-function[163]:0xc676)
    at _ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h6f255f7e971e1b6bE (wasm://wasm/00080b96:wasm-function[152]:0xba77)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h1f5fd5151e12b76fE (wasm://wasm/00080b96:wasm-function[151]:0xb9dd)
    at rust_begin_unwind (wasm://wasm/00080b96:wasm-function[158]:0xc058)
    at _ZN4core9panicking9panic_fmt17h4ed481ff677a9793E (wasm://wasm/00080b96:wasm-function[223]:0x10f95)
    at _ZN4core9panicking5panic17h55c180fe4f8d6e31E (wasm://wasm/00080b96:wasm-function[228]:0x11559)
    at _ZN6engine6engine7exports3inf4wasi6engine36_export_method_engine_inference_cabi17h3e22369a2dd08530E (wasm://wasm/00080b96:wasm-function[28]:0x23d2)
    at inf:wasi/engine#[method]engine.inference (wasm://wasm/00080b96:wasm-function[44]:0x3ae3)

Node.js v20.10.0
```
The same component works with Wasmtime so I'm not sure what is going on here.
Could there be something with the generated bindings that is causing this?
