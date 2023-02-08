use leaf_host_impl::{http::FetchImpl, kv::KvImpl};
use leaf_host_kv::Provider;
use wasi_host::WasiCtx;
use wasi_host_cap_std_sync::WasiCtxBuilder;

pub struct Context {
    pub wasi: WasiCtx,
    pub fetch: FetchImpl,
    pub kv: KvImpl,
}

impl Context {
    pub fn wasi(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    pub fn fetch(&mut self) -> &mut FetchImpl {
        &mut self.fetch
    }
    pub fn kv(&mut self) -> &mut KvImpl {
        &mut self.kv
    }
    pub fn new(req_id: u64, kv: Box<dyn Provider>) -> Self {
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        Self {
            wasi,
            fetch: FetchImpl::new(req_id),
            kv: KvImpl::new("uuid".to_string(), "ns".to_string(), kv),
        }
    }
}
