ifeq ($(Build), debug)
        BUILD_TYPE = debug
else
        BUILD_TYPE = release
        BUILD = "--$(BUILD_TYPE)"
endif

engine_core_wasm=target/wasm32-wasi/${BUILD_TYPE}/engine.wasm
engine_component=target/engine-component.wasm

inference_core_wasm=target/wasm32-wasi/${BUILD_TYPE}/inference.wasm
inference_component=target/inference-component.wasm

config_core_wasm=target/wasm32-wasi/${BUILD_TYPE}/config.wasm
config_component=target/config-component.wasm

### Build core wasm module and utitility targets
build-engine:
	cargo b -p engine ${BUILD} --target wasm32-wasi

show-packages:
	@cargo metadata --format-version=1 --no-deps | jq -r '.packages[].name'

# This target can be useful to inspect the expanded wit-bindgen macros.
cargo-expand:
	cargo expand --target=wasm32-wasi

# Requires cargo install wit-bindgen-cli. This can be used to inspect the
# generated output (Rust source) from the wit file.
.PHONY: wit-bindgen-engine
wit-bindgen-engine:
	wit-bindgen rust wit -w engine-world --out-dir target

.PHONY: wit-bindgen-config
wit-bindgen-config:
	wit-bindgen rust wit -w config-world --out-dir target

.PHONY: wit-bindgen-inference
wit-bindgen-inference:
	wit-bindgen rust wit -w inference-world --out-dir target

wit-bindgen-bindings:
	wit-bindgen rust wit -w inference-world --out-dir target

.PHONY: print-core-wat
print-core-wat:
	wasm-tools print ${engine_core_wasm} | rustfilt

### inference component targets
build-inference:
	cargo b -p inference ${BUILD} --target wasm32-wasi

build-config:
	cargo b -p config ${BUILD} --target wasm32-wasi

### WebAssembly Component Model targets
.PHONY: engine-component
engine-component:
	wasm-tools component new ${engine_core_wasm} \
	--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
	-o ${engine_component}
	@wasm-tools strip $(engine_component) -o $(engine_component)

.PHONY: inference-component
inference-component:
	wasm-tools component new ${inference_core_wasm} \
	--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
	-o ${inference_component}
	wasm-tools strip $(inference_component) -o $(inference_component)

.PHONY: config-component
config-component:
	wasm-tools component new ${config_core_wasm} \
	--adapt wit-lib/wasi_snapshot_preview1.reactor.wasm \
	-o ${config_component}
	wasm-tools strip $(config_component) -o $(config_component)

.PHONY: update-config-component
update-config-component: build-config config-component compose


### Compose a test component which used a static configuration component.
# For manual testing this is useful but for an actual application the
# configuration will be generated by a tool, and the same tool will be used
# to compose the final component.
.PHONY: compose
compose:
	@wasm-tools compose target/inference-component.wasm \
	-d target/engine-component.wasm \
	-d target/config-component.wasm \
	-o target/composed.wasm

.PHONY: print-engine-component-wit
print-engine-component-wit:
	wasm-tools component wit ${engine_component}

.PHONY: print-inference-component-wit
print-inference-component-wit:
	wasm-tools component wit ${inference_component}

.PHONY: print-engine-component-wat
print-engine-component-wat:
	wasm-tools print ${engine_component}

.PHONY: objdump-component
objdump-component:
	@wasm-tools objdump $(engine_component)

### Rust bindings and runtime targets
rust-bindings:
	cargo b -p rust-bindings ${BUILD}

run-rust-bindings:
	@env RUST_BACKTRACE=full WASMTIME_BACKTRACE_DETAILS=1 \
	cargo r -p rust-bindings ${BUILD} -- --component-path "./target/composed.wasm" --model-dir "./models"

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

### Component Generator target
build-generator:
	cargo b -p generator ${BUILD}

CONFIG_NAME=test
.PHONY: generate-component
generate-component:
	cd generator/cli && env RUST_BACKTRACE=full WASMTIME_BACKTRACE_DETAILS=1 \
	cargo r ${BUILD} \
	-- --name ${CONFIG_NAME} \
	--output-dir "../../" \
	--work-dir "../working" \
	--modules-dir "../../target" \
	--model-path=models/llama-2-7b-chat.Q5_K_M.gguf \
	--prompt "<s>[INST] <<SYS>> Only respond with the capital's name in normal case (not uppercase) and nothing else. So only respond with a single word. <</SYS>> What is the capital of Sweden? [/INST]"
	wasm-tools validate -v ${CONFIG_NAME}-composed.wasm

.PHONY: run-generated-component
run-generated-component:
	@env cargo r -p rust-bindings ${BUILD} -- \
	--component-path ${CONFIG_NAME}-composed.wasm \
	--model-dir "models"

.PHONY: start-generator-server
start-generator-server:
	cd generator/api && cargo r ${BUILD} -- \
	--modules-dir "../../target" \
	--work-dir="../working" \
	--output-dir "../working/target" \
	--build-type=${BUILD_TYPE}

.PHONY: generate-component-web
generate-component-web:
	curl -X POST http://localhost:8080/generate \
	-H "Content-Type: application/json" \
	--data-binary @config.json \
	--output test-composed.wasm
	wasm-tools validate -v "${CONFIG_NAME}-composed.wasm"


.PHONY: clean-generator
clean-generator:
	${RM} -rf generator/working

#### Testing targets
.PHONY: rust-all
rust-all: build component run-rust-bindings

