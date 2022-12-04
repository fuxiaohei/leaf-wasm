use super::ComponentContext;
use futures::future::{self, Ready};
use hyper::body::Body;
use hyper::server::conn::AddrStream;
use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use log::info;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ServiceContext {}

impl<'a> Service<Request<Body>> for ServiceContext {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let resp = Response::new(Body::from("Hello, World"));
        let fut = async move { Ok(resp) };
        Box::pin(fut)
    }
}

pub struct ServerContext {
    //component: ComponentContext,
}

impl ServerContext {
    pub fn new(wasm_file: String) -> Result<Self, crate::errors::Error> {
        let _component = ComponentContext::new(wasm_file.as_str())?;
        info!("Load wasm file '{}'", wasm_file);
        Ok(Self {})
    }
}

impl<'addr> Service<&'addr AddrStream> for ServerContext {
    type Response = ServiceContext;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _addr: &'addr AddrStream) -> Self::Future {
        future::ok(ServiceContext {})
    }
}
