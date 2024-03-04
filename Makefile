ifeq ($(Build), debug)
        BUILD_TYPE = debug
else
        BUILD_TYPE = release
        BUILD = "--$(BUILD_TYPE)"
endif

inf_core_wasm=target/wasm32-wasi/${BUILD_TYPE}/inf_wasi.wasm
inf_component=target/inf-wasi-component.wasm

### Build core wasm module and utitility targets
build:
	cargo b ${BUILD} --target wasm32-wasi

# This target can be useful to inspect the expanded wit-bindgen macros.
cargo-expand:
	cargo expand --target=wasm32-wasi

# Requires cargo install wit-bindgen-cli. This can be used to inspect the
# generated output (Rust source) from the wit file.
.PHONY: wit-bindgen
wit-bindgen:
	wit-bindgen rust wit/inf.wit -w inf --exports world=inf

# This target can be used to generate the Rust bindings from the wasi-nn.wit.
wasi-nn-gen:
	wit-bindgen rust wit/wasi-nn.wit

.PHONY: print-wat
print-wat:
	wasm-tools print ${inf_core_wasm} | rustfilt

### WebAssembly Component Model targets
.PHONY: component
component:
	wasm-tools component new ${inf_core_wasm} \
	--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
	-o ${inf_component}

.PHONY: inspect-wit
inspect-wit:
	wasm-tools component wit ${inf_component}

.PHONY: wit-print-wat
wit-print-wat:
	wasm-tools print ${inf_component}

#### Rust bindings and runtime targets
rust-bindings:
	cd rust && cargo build ${BUILD}

run-rust-bindings:
	cd rust && env RUST_BACKTRACE=full  WASMTIME_BACKTRACE_DETAILS=1 cargo run --release

### WasmEdge wasi-nn example
build-wasmedge-wasi-nn-example:
	cargo b --example wasmedge-wasi-nn --target wasm32-wasi --release

PROMPT = "What is LoRA?"
.PHONY: run-wasmedge-wasi-nn-example
run-wasmedge-wasi-nn-example:
	env RUST_BACKTRACE=1 wasmedge --dir .:. \
	--nn-preload llama-chat:GGML:AUTO:models/llama-2-7b-chat.Q5_K_M.gguf \
	"target/wasm32-wasi/release/examples/wasmedge-wasi-nn.wasm" llama-chat \
       	${PROMPT}

### Configuration and pre-requisites targets
.PHONY: install-wasmedge
install-wasmedge:
	curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh \
	       	| bash -s -- --plugin wasi_nn-ggml
	echo "source $HOME/.wasmedge/env"

.PHONY: install-wasm32-wasi
install-wasm32-wasi:
	rustup target add wasm32-wasi

.PHONY: download-model
download-model:
	@mkdir -p models 
	curl -LO https://huggingface.co/TheBloke/Llama-2-7b-Chat-GGUF/resolve/main/llama-2-7b-chat.Q5_K_M.gguf \
		--output models/llama-2-7b-chat.Q5_K_M.gguf
