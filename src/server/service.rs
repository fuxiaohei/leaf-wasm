use crate::errors::Error;
use crate::wasm::{Manager, Pool, Request as LeafRequest, Response as LeafResponse};
use futures::future::{self, Ready};
use hyper::body::Body;
use hyper::server::conn::AddrStream;
use hyper::service::Service;
use hyper::Request;
use hyper::Response;
use once_cell::sync::OnceCell;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

static POOL: OnceCell<Pool> = OnceCell::new();

pub struct ServiceContext {}

impl<'a> Service<Request<Body>> for ServiceContext {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let fut = async move {
            let pool = POOL.get().unwrap();
            let mut worker = pool.get().await.unwrap();
            let worker = worker.as_mut();

            let headers: Vec<(&str, &str)> = vec![];
            let req = LeafRequest {
                method: "GET",
                uri: "/abc",
                headers: &headers,
                body: Some("xxxyyy".as_bytes()),
            };

            let resp: LeafResponse = worker
                .exports
                .handle_request(&mut worker.store, req)
                .unwrap();

            let resp = Response::new(Body::from(resp.body.unwrap()));

            // let resp = Response::new(Body::from("Hello"));

            Ok(resp)
        };
        Box::pin(fut)
    }
}

pub struct ServerContext {}

impl ServerContext {
    pub fn new(wasm_file: String) -> Result<Self, crate::errors::Error> {
        let mgr = Manager::new(wasm_file);
        let pool = Pool::builder(mgr)
            .build()
            .map_err(|e| Error::InitComponentManagerPool(anyhow::anyhow!(e)))?;
        match POOL.set(pool) {
            Ok(_) => Ok(Self {}),
            Err(_) => Err(Error::InitComponentManagerPool(anyhow::anyhow!(
                "Failed to set pool"
            ))),
        }
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
