pub mod http {
    use anyhow::Result;
    use bytes::Bytes;

    pub type Request = http::Request<Bytes>;
    pub type Response = http::Response<Bytes>;

    pub fn error_response(status: http::StatusCode, message: String) -> Result<Response> {
        let mut response = Response::new(message.into());
        *response.status_mut() = status;
        Ok(response)
    }
}

mod http_handler;