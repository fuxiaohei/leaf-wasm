/// Helper for HTTP Request handler
/// base on `http` crates
pub mod http {
    use anyhow::Result;

    /// The HTTP request.
    pub type Request = http::Request<Option<bytes::Bytes>>;
    /// The HTTP response.
    pub type Response = http::Response<Option<bytes::Bytes>>;

    /// Helper function to return Internal Server Error response.
    pub fn internal_server_error(message: String) -> Result<Response> {
        Ok(http::Response::builder()
            .status(500)
            .body(Some(message.into()))?)
    }
}

use sdk_macro;
pub use sdk_macro::http_handler;
