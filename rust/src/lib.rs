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

struct LlmWasi {}
