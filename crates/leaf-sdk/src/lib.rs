mod fetch;

pub mod http {
    pub type Request = http::Request<Option<bytes::Bytes>>;
    pub type Response = http::Response<Option<bytes::Bytes>>;
    pub use super::fetch::fetch;
    pub use super::fetch::FetchOptions;
    pub type Error = super::fetch::HttpError;
}
