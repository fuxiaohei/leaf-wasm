mod http_fetch;
mod http_handler;

pub mod http {
    pub use super::http_fetch::add_to_linker;
    pub use super::http_fetch::FetchImpl;
    pub use super::http_handler::HttpHandler;
    pub use super::http_handler::Request;
    pub use super::http_handler::Response;
}
