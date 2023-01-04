mod clocks;
mod exit;
mod filesystem;

pub use wasi_common::{table::Table, WasiCtx};

type HostResult<T, E> = anyhow::Result<Result<T, E>>;

wasmtime::component::bindgen!({
    path: "./wasi.wit",
    async: true,
});
