use wasi_common::sync::Dir;
use wasmtime::component::ResourceTable;
use wasmtime::{
    component::{bindgen, Component, Linker as ComponentLinker},
    Config, Engine as WasmtimeEngine, Linker, Store,
};

use wasmtime_wasi::DirPerms;
use wasmtime_wasi::FilePerms;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiView;
use wasmtime_wasi_nn::WasiNnCtx;

use wasmtime_wasi_nn::backend::llama_cpp::LlamaCppBackend;
use wasmtime_wasi_nn::InMemoryRegistry;

bindgen!({
    path: "../../wit",
    world: "engine",
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

//#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
//async fn main() -> wasmtime::Result<()> {
fn main() -> wasmtime::Result<()> {
    println!("Rust inf-wasi bindings example!");
    let mut config = Config::new();
    config.wasm_component_model(true);
    //config.async_support(true);
    config.async_support(false);

    let engine = WasmtimeEngine::new(&config)?;
    let bytes = include_bytes!("../../../target/engine-component.wasm");
    let component = Component::from_binary(&engine, bytes)?;
    println!("Loaded component module.");

    let preopen_dir = cap_std::fs::Dir::open_ambient_dir(".", cap_std::ambient_authority())?;
    println!("propen_dir: {:?}", preopen_dir);
    let models_dir = preopen_dir.open_dir("models")?;

    // TODO: remove this after development.
    let model_file = models_dir.exists("llama-2-7b-chat.Q5_K_M.gguf");
    println!("models_file exists: {}", model_file);

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .preopened_dir(models_dir, DirPerms::all(), FilePerms::all(), "models")
        .build();
    println!("Created wasi context.");

    let llama_cpp = LlamaCppBackend::default();
    let registry = InMemoryRegistry::new();
    let wasi_nn = WasiNnCtx::new([llama_cpp.into()], registry.into());
    let command_ctx = CommandCtx {
        table: ResourceTable::new(),
        wasi,
        wasi_nn,
    };
    let mut store = Store::new(&engine, command_ctx);

    //let table = ResourceTable::new();
    //let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    //let ctx = CommandCtx { table, wasi_ctx };

    //let mut store = Store::new(&engine, ctx);
    //let mut linker = wasmtime::Linker::new(&engine);
    let mut component_linker = ComponentLinker::new(&engine);
    wasmtime_wasi::command::sync::add_to_linker(&mut component_linker)?;
    println!("Added wasi to linker.");
    //wasmtime_wasi::sync::add_to_linker(&mut linker)?;
    //wasi_common::sync::add_to_linker(&mut linker, |s| s)?;
    //wasmtime_wasi_nn::wit::add_to_linker(&mut linker)?;
    //let mut linker = Linker::new(&engine);
    wasmtime_wasi_nn::wit::ML::add_to_linker(&mut component_linker, |s: &mut CommandCtx| {
        &mut s.wasi_nn
    })?;
    //wasmtime_wasi_nn::witx::add_to_linker(&mut linker, |s: &mut CommandCtx| &mut s.wasi_nn)?;

    //let (inf, _instance) =
    //   Inf::instantiate_async(&mut store, &component, &component_linker).await?;
    let (inf, _instance) = Engine::instantiate(&mut store, &component, &component_linker)?;

    //println!("inf-wasi version: {}", inf.call_version(&mut store).await?);
    println!("inf-wasi version: {}", inf.call_version(&mut store)?);

    //let result = inf.call_inference(&mut store).await?;
    let result = inf.call_inference(&mut store)?;
    println!("inf-wasi inference: {}", result);
    Ok(())
}
