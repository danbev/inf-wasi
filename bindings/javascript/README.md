## JavaScript bindings
This generates bindings for the composed module for JavaScript.

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
And since we 

With the npm links set up I get the following:
