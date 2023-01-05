use crate::wit::fetch::FetchImpl;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasi_host::WasiCtx;
pub struct Context {
    pub wasi: WasiCtx,
    pub fetch: FetchImpl,
}

impl Context {
    pub fn wasi(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    pub fn fetch(&mut self) -> &mut FetchImpl {
        &mut self.fetch
    }
    pub fn new() -> Self {
        let wasi = WasiCtxBuilder::new().build();
        Self {
            wasi,
            fetch: FetchImpl::new(0),
        }
    }
}
