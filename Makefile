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

build:
	cargo b --release --target wasm32-wasi

.PHONY: print-wat
print-wat:
	wasm-tools print ./target/wasm32-wasi/release/inf_wasi.wasm


component:
	wasm-tools component new -vvv ./target/wasm32-wasi/release/inf_wasi.wasm \
	--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
	-o target/inf-wasi-component.wasm

.PHONY: inspect-wit
inspect-wit:
	wasm-tools component wit target/inf-wasi-component.wasm

.PHONY: wit-print-wat
wit-print-wat:
	wasm-tools print target/inf-wasi-component.wasm

build-rust-bindings:
	cd rust && cargo build --release

run-rust-bindings:
	cd rust && cargo run --release


build-wasmedge-wasi-nn-example:
	cargo b --example wasmedge-wasi-nn --target wasm32-wasi --release

PROMPT = "What is LoRA?"
.PHONY: run-wasmedge-wasi-nn-example
run-wasmedge-wasi-nn-example:
	@env RUST_BACKTRACE=1 wasmedge --dir .:. \
	--nn-preload llama-chat:GGML:AUTO:models/llama-2-7b-chat.Q5_K_M.gguf \
	"target/wasm32-wasi/release/examples/wasmedge-wasi-nn.wasm" llama-chat \
       	${PROMPT}

get-wasi-nn-witx:
	wget https://raw.githubusercontent.com/WebAssembly/wasi-nn/main/wit/wasi-nn.wit
