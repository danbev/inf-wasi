## Python bindings
This generates bindings for the composed module for Python.

Currently this does not work yet.


### Configuration
```console
$ make init-env
$ make install-wasmtime-py
```

### Generate bindings
```console
(p_env) $ make bindings 
Traceback (most recent call last):
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/_func.py", line 268, in enter_wasm
    yield byref(trap)
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/_func.py", line 101, in __call__
    raise WasmtimeError._from_ptr(error)
wasmtime._error.WasmtimeError: error while executing at wasm backtrace:
    0:  0x100 - wit-component:shim!indirect-wasi:cli/terminal-stdin-get-terminal-stdin
    1: 0x254f - wit-component:adapter:wasi_snapshot_preview1!_ZN22wasi_snapshot_preview111descriptors11Descriptors3new17ha2fdde51d30a71bdE
    2: 0x1640 - wit-component:adapter:wasi_snapshot_preview1!_ZN22wasi_snapshot_preview15State11descriptors17h780c546b61bcfc7fE
    3: 0x180c - wit-component:adapter:wasi_snapshot_preview1!fd_write
    4:  0x13a - wit-component:shim!adapt-wasi_snapshot_preview1-fd_write
    5: 0x292b79 - <unknown>!_ZN4wasi13lib_generated8fd_write17ha0fe0cefee614bc7E
    6: 0x28d0aa - <unknown>!_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h9a82cb45fa16fecfE
    7: 0x2a48e9 - <unknown>!_ZN4core3fmt5write17h8483a024db734510E
    8: 0x28e115 - <unknown>!_ZN3std2io5Write9write_fmt17h8553bad7dd23fe65E
    9: 0x291e46 - <unknown>!_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h61263dca9dba1552E
   10: 0x28d5df - <unknown>!_ZN3std9panicking12default_hook17hfbdb6633299d3afbE
   11: 0x2926c9 - <unknown>!_ZN3std9panicking20rust_panic_with_hook17hc97f08b908247b1dE
   12: 0x291a91 - <unknown>!_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hdd638bdaba0c6bccE
   13: 0x2919f7 - <unknown>!_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h5b392607309abb6aE
   14: 0x2920d0 - <unknown>!rust_begin_unwind
   15: 0x29f729 - <unknown>!_ZN4core9panicking9panic_fmt17h9fec598e5939a913E
   16: 0x29fced - <unknown>!_ZN4core9panicking5panic17h711f5cad5118a4ddE
   17: 0xa58d - <unknown>!_ZN7bindgen7bindgen18InterfaceGenerator5types17h1e8eb785ac830c8bE
   18: 0x4f0c - <unknown>!_ZN7bindgen7bindgen10WasmtimePy8generate17h1f87d44abe6d9157E
   19: 0x33c06 - <unknown>!_ZN78_$LT$bindgen..bindings..PythonBindings$u20$as$u20$bindgen..bindings..Guest$GT$8generate17hfe4a73e02a7a0935E
   20: 0x33dea - <unknown>!generate

Caused by:
    python exception

During handling of the above exception, another exception occurred:

Traceback (most recent call last):
  File "<frozen runpy>", line 198, in _run_module_as_main
  File "<frozen runpy>", line 88, in _run_code
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/bindgen/__main__.py", line 40, in <module>
    main()
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/bindgen/__main__.py", line 30, in main
    files = generate(name, contents)
            ^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/bindgen/__init__.py", line 144, in generate
    result = root.generate(store, name, component)
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/bindgen/generated/__init__.py", line 288, in generate
    ret = self.lift_callee0(caller, ptr, len0, ptr1, len2)
          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/_func.py", line 91, in __call__
    with enter_wasm(store) as trap:
  File "/usr/lib64/python3.12/contextlib.py", line 158, in __exit__
    self.gen.throw(value)
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/_func.py", line 274, in enter_wasm
    maybe_raise_last_exn()
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/_func.py", line 284, in maybe_raise_last_exn
    raise exn
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/_func.py", line 194, in trampoline
    pyresults = func(*pyparams)
                ^^^^^^^^^^^^^^^
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/bindgen/generated/__init__.py", line 200, in lowering16_callee
    ret = import_object.terminal_stdin.get_terminal_stdin()
          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/danielbevenius/work/ai/inf-wasi/bindings/python/p_env/lib64/python3.12/site-packages/wasmtime/bindgen/__init__.py", line 95, in get_terminal_stdin
    raise NotImplementedError
NotImplementedError
make: *** [Makefile:13: bindings] Error 1

```
