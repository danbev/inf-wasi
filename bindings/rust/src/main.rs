use wasmtime::component::ResourceTable;
use wasmtime::{
    component::{bindgen, Component, Linker as ComponentLinker},
    Config, Engine as WasmtimeEngine, Store,
};

use std::path::Path;
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
    world: "engine-world",
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

fn main() -> wasmtime::Result<()> {
    println!("Rust inf-wasi bindings example!");
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(false);

    let engine = WasmtimeEngine::new(&config)?;
    let bytes = include_bytes!("../../../target/engine-component.wasm");
    let component = Component::from_binary(&engine, bytes)?;
    println!("Loaded component module.");

    let path = Path::new(".");
    let preopen_dir = cap_std::fs::Dir::open_ambient_dir(path, cap_std::ambient_authority())?;
    println!("prepen_dir: {}", path.display());
    let models_dir = preopen_dir.open_dir("models")?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .preopened_dir(models_dir, DirPerms::all(), FilePerms::all(), "models")
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
    wasmtime_wasi::command::sync::add_to_linker(&mut component_linker)?;
    wasmtime_wasi_nn::wit::ML::add_to_linker(&mut component_linker, |s: &mut CommandCtx| {
        &mut s.wasi_nn
    })?;

    let (engine, _instance) = EngineWorld::instantiate(&mut store, &component, &component_linker)?;

    println!(
        "engine version: {}",
        engine.interface0.call_version(&mut store)?
    );

    let prompt = "TODO: Add this to config. Not used at the moment!";
    let result = engine.interface0.call_inference(&mut store, &prompt)?;
    println!("engine inference: {}", result);
    Ok(())
}
