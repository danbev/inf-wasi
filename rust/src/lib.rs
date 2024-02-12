use wasmtime::component::bindgen;
use wasmtime_wasi::preview2::{ResourceTable, WasiCtx, WasiView};

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
