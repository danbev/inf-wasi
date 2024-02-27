## wasi-nn tensor wrong type issue

This is the error that I'm seeing when trying to run the rust-binding which
tries to load a component that uses wasi-nn. 
```
$ make run-rust-bindings 
cd rust && cargo run --release
warning: unused import: `wasmtime::component::bindgen`
 --> src/lib.rs:1:5
  |
1 | use wasmtime::component::bindgen;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `rust` (lib) generated 1 warning (run `cargo fix --lib -p rust` to apply 1 suggestion)
    Finished release [optimized] target(s) in 0.11s
     Running `target/release/rust`
Error: import `wasi:nn/tensor` has the wrong type

Caused by:
    0: instance export `tensor` has the wrong type
    1: expected resource found nothing
make: *** [Makefile:50: run-rust-bindings] Error 1
```
So lets inspect the imports of the `wasi:nn` module:
```
$ make print-wat | rustfilt > output-wat
```console
(import "wasi:nn/tensor" "[resource-drop]tensor" (func $<inf_wasi::wit::wasi::wasi::nn::tensor::Tensor as wit_bindgen::WasmResource>::drop::drop (;0;) (type 0)))
   26   (import "wasi:nn/graph" "[resource-drop]graph" (func $<inf_wasi::wit::wasi::wasi::nn::graph::Graph as wit_bindgen::WasmResource>::drop::drop (;1;) (type 0)))
   27   (import "wasi:nn/inference" "[resource-drop]graph-execution-context" (func $<inf_wasi::wit::wasi::wasi::nn::inference::GraphExecutionContext as wit_bindgen::WasmResource>::drop::drop (;2;) (type 0)))
   28   (import "wasi:nn/errors" "[resource-drop]error" (func $<inf_wasi::wit::wasi::wasi::nn::errors::Error as wit_bindgen::WasmResource>::drop::drop (;3;) (type 0)))
   29   (import "wasi:nn/tensor" "[constructor]tensor" (func $inf_wasi::wit::wasi::wasi::nn::tensor::Tensor::new::wit_import (;4;) (type 5)))
   30   (import "wasi:nn/inference" "[method]graph-execution-context.set-input" (func $inf_wasi::wit::wasi::wasi::nn::inference::GraphExecutionContext::set_input::wit_import (;5;) (type 6)))
   31   (import "wasi:nn/inference" "[method]graph-execution-context.compute" (func $inf_wasi::wit::wasi::wasi::nn::inference::GraphExecutionContext::compute::wit_import (;6;) (type 1)))
   32   (import "wasi:nn/inference" "[method]graph-execution-context.get-output" (func $inf_wasi::wit::wasi::wasi::nn::inference::GraphExecutionContext::get_output::wit_import (;7;) (type 7)))
   33   (import "wasi:nn/graph" "[method]graph.init-execution-context" (func $inf_wasi::wit::wasi::wasi::nn::graph::Graph::init_execution_context::wit_import (;8;) (type 1)))
   34   (import "wasi:nn/graph" "load-by-name" (func $inf_wasi::wit::wasi::wasi::nn::graph::load_by_name::wit_import (;9;) (type 3)))
   35   (import "wasi_snapshot_preview1" "fd_write" (func $wasi::lib_generated::wasi_snapshot_preview1::fd_write (;10;) (type 8)))
   36   (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;11;) (type 2)))
   37   (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;12;) (type 2)))
   38   (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;13;) (type 0)))
```
_wip_

