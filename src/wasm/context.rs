use crate::wit::fetch::FetchImpl;
use wasi_host::WasiCtx;
use wasi_host_cap_std_sync::WasiCtxBuilder;
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
    pub fn new(req_id: u64) -> Self {
        let wasi = WasiCtxBuilder::new().build();
        Self {
            wasi,
            fetch: FetchImpl::new(req_id),
        }
    }
}
