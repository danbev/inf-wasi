use wasmtime::component::ResourceTable;
use wasmtime::{
    component::{bindgen, Component, Linker as ComponentLinker},
    Config, Engine as WasmtimeEngine, Store,
};

use anyhow::{Context, Result};
use std::path::PathBuf;
use wasmtime_wasi::DirPerms;
use wasmtime_wasi::FilePerms;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiView;
use wasmtime_wasi_nn::backend::llama_cpp::LlamaCppBackend;
use wasmtime_wasi_nn::InMemoryRegistry;
use wasmtime_wasi_nn::WasiNnCtx;

bindgen!({
    path: "../../wit",
    world: "inference-world",
    async: false,
});

struct CommandCtx {
    table: ResourceTable,
    wasi: WasiCtx,
    wasi_nn: WasiNnCtx,
}

impl WasiView for CommandCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub struct WasmtimeInference {
    store: Store<CommandCtx>,
    inference_world: InferenceWorld,
}

impl WasmtimeInference {
    pub fn new(component_path: PathBuf, model_path: PathBuf) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(false);

        let engine = WasmtimeEngine::new(&config).context("Failed to create WasmtimeEngine")?;

        let component =
            Component::from_file(&engine, &component_path).context("Failed to create Component")?;
        println!("Loaded component module.");

        let model_dir =
            cap_std::fs::Dir::open_ambient_dir(&model_path, cap_std::ambient_authority())
                .context("Failed to open ambient dir")?;
        println!("model_dir: {}", &model_path.display());

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(model_dir, DirPerms::all(), FilePerms::all(), "models")
            .build();

        let llama_cpp = LlamaCppBackend::default();
        let registry = InMemoryRegistry::new();
        let wasi_nn = WasiNnCtx::new([llama_cpp.into()], registry.into());
        let command_ctx = CommandCtx {
            table: ResourceTable::new(),
            wasi,
            wasi_nn,
        };
        let mut store = Store::new(&engine, command_ctx);

        let mut component_linker = ComponentLinker::new(&engine);
        wasmtime_wasi::command::sync::add_to_linker(&mut component_linker)
            .context("Failed to add wasi to linker")?;
        wasmtime_wasi_nn::wit::ML::add_to_linker(&mut component_linker, |s: &mut CommandCtx| {
            &mut s.wasi_nn
        })
        .context("Failed to add wasi_nn to linker")?;

        let (inference_world, _instance) =
            InferenceWorld::instantiate(&mut store, &component, &component_linker)
                .context("Failed to instantiate InferenceWorld")?;
        Ok(Self {
            store,
            inference_world,
        })
    }

    pub fn compute(self, prompt: Option<&str>) -> String {
        let result = self
            .inference_world
            .call_compute(self.store, prompt)
            .context("Failed to call compute")
            .unwrap();
        result
    }
}
