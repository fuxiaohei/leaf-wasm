#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// failed to init wasmtime engine
    #[error("failed to init wasmtime engine: {0}")]
    InitEngine(anyhow::Error),

    /// failed to load wasmtime component
    #[error("failed to load wasmtime component: {0}, file: {1}")]
    ReadWasmComponent(anyhow::Error, String),

    /// failed to instantiate wasmtime component
    #[error("failed to instantiate wasmtime component: {0}")]
    InstantiateWasmComponent(anyhow::Error),

    /// failed to invoke component export function
    #[error("failed to invoke component export function: {0}")]
    InvokeComponentExportFunction(anyhow::Error),
}
