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

prompt = "What is LoRA?"
.PHONY: run-example
run-example:
	@echo ${LD_LIBRARY_PATH}
	env RUST_BACKTRACE=1 WASMEDGE_LOG_LEVEL=debug \
	wasmedge --dir .:. \
	--nn-preload llama-chat:GGML:AUTO:models/llama-2-7b-chat.Q5_K_M.gguf \
       	"target/wasm32-wasi/release/llm_wasi.wasm" llama-chat ${prompt}

component:
	wasm-tools component new -vvv ./target/wasm32-wasi/release/llm_wasi.wasm \
	--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
	-o target/llm-wasi-component.wasm

inspect-wit:
	wasm-tools component wit target/llm-wasi-component.wasm
