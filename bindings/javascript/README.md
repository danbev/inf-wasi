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
