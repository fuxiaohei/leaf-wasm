mod fetch;

pub mod http {
    use super::fetch;
    use bytes::Bytes;

    pub type Request = http::Request<Option<Bytes>>;
    pub type Response = http::Response<Option<Bytes>>;

    pub use fetch::fetch;
    pub use fetch::FetchOptions;
    pub use fetch::RedirectPolicy;
    pub type Error = fetch::HttpError;
}

// Re-export leaf_sdk_macro,
// make all functions in leaf_sdk
pub use leaf_sdk_macro::http_main;
