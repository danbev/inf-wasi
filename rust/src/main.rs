use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine as WasmtimeEngine, Store,
};
use wasmtime_wasi::preview2::command::add_to_linker;
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../wit",
    world: "inf",
    async: true,
});

struct CommandCtx {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
}

impl WasiView for CommandCtx {
    fn table(&self) -> &ResourceTable {
        &self.table
    }
    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = WasmtimeEngine::new(&config)?;
    let bytes = include_bytes!("../../target/inf-wasi-component.wasm");
    let component = Component::from_binary(&engine, bytes)?;

    let table = ResourceTable::new();
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let ctx = CommandCtx { table, wasi_ctx };

    let mut store = Store::new(&engine, ctx);
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;

    let (inf, _instance) = Inf::instantiate_async(&mut store, &component, &linker).await?;

    println!("inf-wasi version: {}", inf.call_version(&mut store).await?);
    let result = inf.call_inference(&mut store).await?;
    println!("inf-wasi inference: {}", result);
    Ok(())
}
