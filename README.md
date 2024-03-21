## inf-wasi
This is a proof of concept (POC) for investigating the possiblity of combining
wasi-nn with the WebAssembly Component Model. The idea is to create a
interface using WebAssembly interface types (WIT) engine, a configuration, 
and an inference component.

```
 Engines components        Inference component
 +------------+            +------------+
 | llama.cpp  |            | engine     |
 +------------+            | config     |
                           +------------+
 +------------+
 | OpenVINO   |
 +------------+
 ...
 Other backends
```
We have defined an interface for the engine and inference in
[inf.wit](wit/inf.wit).

A/the engine is implemented in [engine.rs](engine/src/engine.rs) and the
inferenece component is implemented in [inference.rs](inference/src/lib.rs).

These components are "pre-baked" and are component modules that can be used
to compose a user specific inference component. 

The idea is that each user will have different configuration so the idea is to
combine the configuration with the a chosen engine above to create an inference
component for that user.
```
  Engine component
  +------------+ 
  | llama.cpp  |-----+     Inference component
  +------------+     |     +-------------+
                     +---->| MyInference | 
  +------------+     |     +-------------+
  | MyConfig   |-----+
  +------------+
  Config component
```
Actually having separate engine components might be unnecessary as this could
also be done via the configuration and select the backend to use from that but
a engine component is needed for the composition nonetheless. After going back
and forth on this I think it is better to have I single engine component and
then have a configuration that is specific to each perhaps. But having multiple
modules might lead to maintainability issues.

We can then generate bindings for the composed module for different languages:
```
  +------------+  ----------> Rust
  | MyInference|  ----------> JavaScript
  +------------+  ----------> Python
```
Example of [Rust](bindings/rust), and [JavaScript](bindings/javascript) are
available but the JavaScript bindings need a shim for `wasi:nn` which is not
available yet. The Python bindings are not working yet.

To show this in action we can perform the following steps:
#### 1. Build the engine and inference components
```console
$ make build-llama-cpp-engine build-inference
$ make llama-cpp-engine-component inference-component
```
#### 2. Generate the configuration component
```console
$ make generate-config-component 
cd generator && env RUST_BACKTRACE=full WASMTIME_BACKTRACE_DETAILS=1 \
cargo r -p generator --bin wasm-generator "--release" \
-- --name "sample" --model-path=models/llama-2-7b-chat.Q5_K_M.gguf \
--output-dir "working/target" \
--modules-dir "../target" \
--prompt "<s>[INST] <<SYS>> Only respond with the capital's name in normal case (not uppercase) and nothing else. So only respond with a single word. <</SYS>> What is the capital of Sweden? [/INST]"
   Compiling generator v0.1.0 (/home/danielbevenius/work/ai/inf-wasi/generator)
    Finished release [optimized] target(s) in 1.59s
     Running `/home/danielbevenius/work/ai/inf-wasi/target/release/wasm-generator --name sample --model-path=models/llama-2-7b-chat.Q5_K_M.gguf --output-dir working/target --modules-dir ../target --prompt '<s>[INST] <<SYS>> Only respond with the capital'\''s name in normal case (not uppercase) and nothing else. So only respond with a single word. <</SYS>> What is the capital of Sweden? [/INST]'`
Created workspace 'working'
Building workspace...done
Created webassembly component: "working/target/sample-config-component.wasm"
Composed into webassembly component:
"/home/danielbevenius/work/ai/inf-wasi/generator/working/target/sample-composed.wasm"
```
Notice that this command line tool `wasm-generator` is actually standalone from
the project and could be run on a server where it has access to the engine and
inference components but nothing else.

#### 3. Use the generated component in Rust
The Rust [bindings](bindings/rust/src/lib.rs) uses wasmtime to load the
composed component and invoke the `compute` function of the inference component.
The [main](bindings/rust/src/main.rs) uses this libaray which is what is
called below.
```console
$ make run-generated-component 
   Compiling wasmtime-wasi v19.0.0 (/home/danielbevenius/work/wasm/wasmtime/crates/wasi)
   Compiling rust-bindings v0.1.0 (/home/danielbevenius/work/ai/inf-wasi/bindings/rust)
    Finished release [optimized] target(s) in 31.70s
     Running `target/release/rust-bindings --component-path generator/working/target/sample-composed.wasm --model-dir models`
Inference Component Running inference
LlamaCppBackend: model_path: "models/llama-2-7b-chat.Q5_K_M.gguf"
llama_model_loader: loaded meta data with 19 key-value pairs and 291 tensors from models/llama-2-7b-chat.Q5_K_M.gguf (version GGUF V2)
llama_model_loader: - tensor    0:                token_embd.weight q5_K     [  4096, 32000,     1,     1 ]
...
llm_load_vocab: special tokens definition check successful ( 259/32000 ).
llm_load_print_meta: format           = GGUF V2
llm_load_print_meta: arch             = llama
llm_load_print_meta: vocab type       = SPM
llm_load_print_meta: n_vocab          = 32000
llm_load_print_meta: n_merges         = 0
llm_load_print_meta: n_ctx_train      = 4096
llm_load_print_meta: n_embd           = 4096
llm_load_print_meta: n_head           = 32
llm_load_print_meta: n_head_kv        = 32
llm_load_print_meta: n_layer          = 32
llm_load_print_meta: n_rot            = 128
llm_load_print_meta: n_gqa            = 1
llm_load_print_meta: f_norm_eps       = 0.0e+00
llm_load_print_meta: f_norm_rms_eps   = 1.0e-06
llm_load_print_meta: f_clamp_kqv      = 0.0e+00
llm_load_print_meta: f_max_alibi_bias = 0.0e+00
llm_load_print_meta: n_ff             = 11008
llm_load_print_meta: rope scaling     = linear
llm_load_print_meta: freq_base_train  = 10000.0
llm_load_print_meta: freq_scale_train = 1
llm_load_print_meta: n_yarn_orig_ctx  = 4096
llm_load_print_meta: rope_finetuned   = unknown
llm_load_print_meta: model type       = 7B
llm_load_print_meta: model ftype      = mostly Q5_K - Medium
llm_load_print_meta: model params     = 6.74 B
llm_load_print_meta: model size       = 4.45 GiB (5.68 BPW) 
llm_load_print_meta: general.name     = LLaMA v2
llm_load_print_meta: BOS token        = 1 '<s>'
llm_load_print_meta: EOS token        = 2 '</s>'
llm_load_print_meta: UNK token        = 0 '<unk>'
llm_load_print_meta: LF token         = 13 '<0x0A>'
llm_load_tensors: ggml ctx size =    0.11 MiB
llm_load_tensors: mem required  = 4560.97 MiB
...................................................................................................
llama_new_context_with_model: n_ctx      = 512
llama_new_context_with_model: freq_base  = 10000.0
llama_new_context_with_model: freq_scale = 1
llama_new_context_with_model: kv self size  =  256.00 MiB
llama_build_graph: non-view tensors processed: 676/676
llama_new_context_with_model: compute buffer total size = 73.57 MiB
Engine model_path: models/llama-2-7b-chat.Q5_K_M.gguf
Engine prompt: <s>[INST] <<SYS>> Only respond with the capital's name in normal case (not uppercase) and nothing else. So only respond with a single word. <</SYS>> What is the capital of Sweden? [/INST]
Result:   Stockholm
```

And the idea would be that the same could be possible from other languages like
Python, JavaScript, Java, etc.

### Motivation
There are currently offerings available for running inference locally, like
[llamafile], [run-llama.sh], [llmstudio], [llamaedge], [ollama], [localllm],
[Chat with RTX], and possible others as this is a fast moving field. The target
user of these are users wanting to run llm inference on their local machine for
reasons like privacy (not sharing their data with an LLM inference service
provider), or wanting to avoid the cost of a LLM inference provider.

The intention of `inf-wasi` it to cater for developers that want to run
inference in their applications, and simliar to the above users they also want
the privacy and avoid the cost, but also want to run the inference in a secure
manner since they will be using it in their own applications. These user might
also be interested in being able to run inf-wasi from different programming
languages, like Rust, Python, JavaScript, Java, etc. By using the Web Assembly
Component Model we can provide a single component interface that can be used
from different languages by generating bindings for those languages. This allows
applications to use the llm inference in a secure manner, which the wasm
sandboxing provides.

By abstracting the inference from the concrete wasi-nn specification we can
allow for different implementations of the inference engine. Lets say that
`wasm64-wasi` is released, that could mean that we are able to run the inference
directly in "pure" wasm without the need for wasi-nn (if it is still possible
to access hardware accellerators that is).

When `wasm64-wasi` is released/supported it would enable is packaging the models
into modules and then have a single component with everything needed to run the
inference. This would be a big win as currently the models need to handles
separately from from the .wasm. This would simplify deployment is there was only
a single .wasm file to deploy. Having a separate model file can also perhaps be
viewed as an attach vector which could be manipluated by an attacker. Having
the model in a component might been that we could sign the .wasm as a whole as
well to verify that it has not been tampered with.

When choosing a model, or switching to a different model, one might want to
first test the model out and verify that the prompt to be used work with that
model. The same prompt might not work without tweaking for another model. The
idea is to have a separate component model for the configuration of the
inference engine which allows it to be updated and then composed/recomposed
with the inference engine component.

Currently this is all being done on the command line but the idea is that this
could be done on the server side and controlled via a graphical user interface.

### Example usage
So the idea is to create a WebAssembly interface types definition for the
inference engine/runtime. The engine will use wasi-nn to do the actual compute
inference. This examples used Wasmtime embedded to run the examples. We are
using a wasmtime-wasi-nn backend for llama.cpp which was written as part of this
POC, and is just an example so far.

There are three components that are included in the produces end user components
which are:
* engine component
* configuration component
* inference component

The engine component is what uses wasi-nn to run the inference. The
configuration component is what configures the engine with things like the
model (path) to use, the prompt and other configuration properties in the
future. The inference component is the component that is used to combine these
two components into a single component.

So, lets first build the engine and inference components:
```console
$ make build-llama-cpp-engine build-inference
$ make llama-cpp-engine-component inference-component
```

So, we now have two components that can be composed into a single component and
we do this by including a configuration component. 

The configuration component is generated by the [generator](./generator) which
is a library, and also has a command line [tool](bindings/rust/src/main.rs). The
idea is that the generation could be run on a server, where is has access to the
 two component modules above but nothing apart from them. 

We can generate the configuration component using the following command:
```console
$ make generate-config-component 
    Finished release [optimized] target(s) in 0.09s
     Running `/home/danielbevenius/work/ai/inf-wasi/target/release/wasm-generator --name sample --model-path=models/llama-2-7b-chat.Q5_K_M.gguf --output-dir working/target --modules-dir ../target --prompt '<s>[INST] <<SYS>> Only respond with the capital'\''s name in normal case (not uppercase) and nothing else. <</SYS>> What is the capital of Sweden?1} [/INST]'`
Generating component: config GenConfig { name: "sample", model_path: "models/llama-2-7b-chat.Q5_K_M.gguf", prompt: "<s>[INST] <<SYS>> Only respond with the capital's name in normal case (not uppercase) and nothing else. <</SYS>> What is the capital of Sweden?1} [/INST]", build_type: Debug, modules_dir: "../target", output_dir: "working/target" }
Created workspace 'working'
Building workspace...done
Created webassembly component: "working/target/sample-config-component.wasm"
Composed into webassembly component:
"/home/danielbevenius/work/ai/inf-wasi/generator/working/target/sample-composed.wasm"
```

We can then we can run the composed component from above using the following
command:
```console
$ make run-generated-component 
    Finished release [optimized] target(s) in 0.17s
     Running `target/release/rust-bindings --component-path generator/working/target/sample-composed.wasm --model-dir models`
Loaded component module.
model_dir: models
Running inference
LlamaCppBackend: model_path: "models/llama-2-7b-chat.Q5_K_M.gguf"
llama_model_loader: loaded meta data with 19 key-value pairs and 291 tensors from models/llama-2-7b-chat.Q5_K_M.gguf (version GGUF V2)
llama_model_loader: - tensor    0:                token_embd.weight q5_K     [  4096, 32000,     1,     1 ]
llama_new_context_with_model: compute buffer total size = 73.57 MiB
...
Engine model_path: models/llama-2-7b-chat.Q5_K_M.gguf
Engine prompt: <s>[INST] <<SYS>> Only respond with the capital's name in normal case (not uppercase) and nothing else. <</SYS>> What is the capital of Sweden?1} [/INST]
LlamaCppExecutionContext: compute...
Result:   Sure! The capital of Sweden is Stockholm.
```

### Tasks
- [] Design Inference Interface  
- [] Bindings  
  - [] Rust bindings and implementation  
  - [] Python bindings and implementation  
  - [] JavaScript bindings and implementation  
  - [] Java (if possible) bindings and implementation  
- [] Add wasm64-wasi support (to enable models to be packaged as modules)  
- [] Add interface for models in wasm components  
- [] Investigate if running inference using wasm64-wasi is possible and that access   
   to hardware accellerators is possible directly in this case  
- [] Investigate MultiModal Models (like visual->text, speach->text, text->speach)  
- [] Investigate lightweight LLM/MML for resource constrained devices
- [] Investigate adding ml-bom (Machine Learning Bill of Materials)

### Configuration for WasmEdge example
This is a standalone example that used WasmEdge and was used initially as a
working example before the WebAssembly Component Model was used. I'll keep it
around for now as it might be useful for testing and perhaps later same
component generated could be used in either WasmEdge or Wasmtime (or any other
wasm runtime that supports wasi-nn and the component model).

We need to install [WasmEdge](https://wasmedge.org/) which is the wasm runtime
that will be used:
```console
$ make install-wasmedge
```

We also need to download a LLM model to use:
```
$ make download-model
```

### Building WasmEdge with CUDA support and wasi-nn
```console
$ source cude-env.sh
$ cd /path/to/WasmEdge
$ mkdir build && cd build
$ cmake .. -DWASMEDGE_PLUGIN_WASI_NN_BACKEND=ggml -DWASMEDGE_PLUGIN_WASI_NN_GGML_LLAMA_CUBLAS=ON -DCMAKE_CUDA_ARCHITECTURES=52;61;70 -DCMAKE_INSTALL_PREFIX=dist
$ make -j8
$ cmake --install . --prefix /home/danielbevenius/.local-wasmedge
```
So the above will configure and build wasmedge into the dist directory. We
can now configure it so that wasmedge is used from there.

### Implementation

#### Configuration
So the idea is to have a module that is pretty much self contained, apart from
the model file(s) that it needs to run. The should be one component/world for
the inference engine itself. This would mostly be a static module that is used
to componse the end users module.

There would also be a config component which contains the configuration for the
engine. This could contains information like the model path, configuration
options for the engine, the prompt to be used with the engine. The prompt being
part of the configuration might sound strange but simply switching from one
model to another might require a different prompt. I'm imaging that using a
separate tool for testing out a model and writing the prompt for that specific
model and any other tuning parameters like temperature etc would be collected
into the configuration component, which would then be regenereated and componsed
 with the inference engine component to create the end users module.

__wip_
```
modules/inf-wasi-component.wasm
```

#### Llama.cpp support
Currently WasmEdge has a plugin for llama.cpp and they have created their own
fork of wasm-nn to add the Ggml graph encoding. I've been working on adding
llama.cpp support to Wasmtime and would also need to make this change. Also
the current version of WasmEdge's fork uses the older .witx file format and not
the newer .wit format. I've updated the [wasi-nn rust bindings] to generate Rust
code for the new .wit format and it also manually add the ggml graph encoding
(which I actually called gguf but think that might be a mistake).

#### Wasmtime support
To build wasi-nn in wasmtime the following feature needs to be enabled:
```console
$ cargo b --features="llama_cpp"
```
The wasi-nn spec is a submodule in crates/wasi-nn and I've currently manually
updated the wasi-nn.wit file to include the ggml graph encoding.


[wasi-nn-pr]: https://github.com/WebAssembly/wasi-nn/pull/66
[Ggml]: https://github.com/second-state/wasmedge-wasi-nn/blob/ggml/rust/src/graph.rs
[wasi-nn rust bindings]: https://github.com/bytecodealliance/wasi-nn.git


[llamafile]: https://github.com/Mozilla-Ocho/llamafile
[run-llama.sh]: https://www.secondstate.io/articles/run-llm-sh/
[llmstudio]: https://lmstudio.ai/
[llamaedge]: https://www.secondstate.io/LlamaEdge/
[wasmtime]: https://github.com/bytecodealliance/wasmtime/commits?author=danbev
[wasm-tools]: https://github.com/bytecodealliance/wasm-tools/graphs/contributors
[wasmtime-py]: https://github.com/bytecodealliance/wasmtime-py/commits?author=danbev
[llm-chain]: https://github.com/sobelio/llm-chain/commits?author=danbev
[llama.cpp]: https://github.com/ggerganov/llama.cpp/commits?author=danbev
[seedwing]: https://github.com/seedwing-io/seedwing-policy/pull/237
[witx-bindgen]: https://github.com/bytecodealliance/wasi/tree/main/crates/witx-bindgen.
[wasn-nn wit]:  https://github.com/WebAssembly/wasi-nn/tree/main/wit
[generated.rs]: https://github.com/second-state/wasmedge-wasi-nn/blob/ggml/rust/src/generated.rs
[trustification]: https://github.com/trustification/trustification
[llava]: https://github.com/danbev/learning-ai/blob/main/notes/llava.md
[ollama]:  https://ollama.com/
[localllm]: https://github.com/googlecloudplatform/localllm
[chat with rtx]: https://www.nvidia.com/en-us/ai-on-rtx/chat-with-rtx-generative-ai/
