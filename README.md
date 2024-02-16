## inf-wasi
This is Web Component Module Component for Languge Model inference (inf). This
can be used for Large Language Models (LLM), and Large Multimodal Models (LMM)
(like [LLaVA]) inference.

### Background
I've been focused on finding a use case for AI in [Trustification], but instead
of trying to come up (forcing) a usage in our project, perhaps we should shift
our focus to enabling our customers to use AI in their application, and do so
in a secure manner. This is the motivation for this project.

### Motivation
There are currently offerings available for running inference locally, like
[llamafile], [run-llama.sh], [llmstudio], [llamaedge], [ollama], and possible
others as this is a fast moving field. The target user of these are user wanting
to run llm inference on their local machine for reasons like privacy (not
sharing their data with an LLM inference service provider), or wanting to avoid
the cost of a LLM inference provider.

The intention of `inf-wasi` it to cater for developers that want to run
inference in their applications, and simliar to the above users they also want
the privacy and avoid the cost, but also want to run the inference in a secure
manner since they will be using it in their own applications. These user might
also interested in being able to run inf-wasi from different programming
languages, like Rust, Python, JavaScript, Java, etc. By using the Web Assembly
 Component Model we can provide a single component interface that can be used
from different languages by generating bindings for those languages.

By abstracting the inference from the concrete wasi-nn specification we can
allow for different implementations of the inference engine. Lets say that
`wasm64-wasi` is released, that could mean that we are able to run the inference
directly in "pure" wasm without the need for wasi-nn (if it is still possible
to access hardware accellerators that is).

Another thing that `wasm64-wasi` would enable is packaging the models into
modules and then have a single component with everything needed to run the
inference. This would be a big win as currently the models need to handles
separately from from the .wasm. This would simplify deployment is there was only
a single .wasm file to deploy. Having a separate model file can also perhaps be
viewed as an attach vector which could be manipluated by an attacker. Having
the model in a component might been that we could sign the .wasm as a whole as
well to verify that it has not been tampered with.

Another motivation is that the same .wasm component module can be used in
different languages, for example generate bindings for Rust, JavaScript, Python,
Java. This allows applications to use the llm inference in a secure manner,
which the wasm sandboxing provides.

We have the opportunity to create a new interfaces that is easier to use and
understand. The wasi-nn interface is quite low level and it would be nice to
have a higher level interface that is easier to use.

While the composition of modules is currently being done on the command line it
could equally be done on the serverside, and controlled via a graphical user
interface.

Doing this work would enable us to become part of this new (for us) space which
will become more and more important moving forward. With the rise of LLM
inference we are also seeing Large Multimodal Models (LMM) which might be even 
more useful in the future. These models can enable the creation of applications
that can take an image an input and produce text as ouput which describes the
image. But there are further applications of this with are related to agents
being able to understand/interpret the a GUI and then be able to interact with
it. 

### Current status of this project
At the time of writing this project just contains a skeleton of the ideas above
to make it easier to explain the ideas and get feedback from others if this is
worth exploring further.

### WebAssembly Component Model
So the idea is to create a WebAssembly interface types definition for the
inference engine/runtime. The engine will use wasi-nn to do the actual compute
inference. 

Now, a simple wasi-nn example can be found in the examples directory:
```console
$ make build-wasmedge-wasi-nn-example
$ make run-wasmedge-wasi-nn-example
```

So we are able to compile this to wasm and run it using wasmedge. But I'd also
like to be able transform this core wasm module into a web component module
using the wasm-tools component new command:
```console
$ make component 
wasm-tools component new ./target/wasm32-wasi/release/inf_wasi.wasm \
--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
-o target/inf-wasi-component.wasm
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

### Configuration
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
