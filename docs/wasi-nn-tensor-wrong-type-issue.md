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
Hmm, if we look at the tensor import and the package names they have `inf_wasi`
in them which should not be the case. This is because I thougt it would be
clearer to generate, using wit-bindgen, the wasi-nn bindings in a separate
source file and then include it as I would be able to inspect the generated
code easier. But perhaps I should just use the wit-bindgen macro directly in
the Rust source file and see if that works.

Lets also take a look at the wit from the component.wasm:
```console
$ make inspect-wit 
wasm-tools component wit target/inf-wasi-component.wasm
package root:component;

world root {
  import wasi:nn/tensor;
  import wasi:nn/errors;
  import wasi:nn/inference;
  import wasi:nn/graph;
  import wasi:cli/environment@0.2.0;
  import wasi:cli/exit@0.2.0;
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdin@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:cli/stderr@0.2.0;
  import wasi:clocks/wall-clock@0.2.0;
  import wasi:filesystem/types@0.2.0;
  import wasi:filesystem/preopens@0.2.0;

  export version: func() -> string;
  export inference: func() -> string;
}
```

So after some digging around I realized that the version of wasi-nn.wit that is
used in wasmtime-wasi-nn is an older version compared to the one that I updated
wasi-nn with. At the moment wasi-nn is still using the old witx format and I'v
updated that to use the new wit format. I just used the latest version available
and hoped that would work. But that lead to the wrong type issue. So what I've
tried now is using the same version that wasmtime-wasi-nn uses and that seems to
work.

_wip_

