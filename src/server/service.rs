use crate::errors::Error;
use crate::wasm::{Manager, Pool, Request as LeafRequest, Response as LeafResponse};
use futures::future::{self, Ready};
use hyper::{body::Body, server::conn::AddrStream, service::Service, Request, Response};
use log::warn;
use once_cell::sync::OnceCell;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

static POOL: OnceCell<Pool> = OnceCell::new();

pub struct ServiceContext {}

impl<'a> Service<Request<Body>> for ServiceContext {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let fut = async move {
            let pool = POOL.get().unwrap();
            let mut worker = pool.get().await.unwrap();
            let worker = worker.as_mut();

            let mut headers: Vec<(&str, &str)> = vec![];
            let req_headers = req.headers().clone();
            req_headers.iter().for_each(|(k, v)| {
                headers.push((k.as_str(), v.to_str().unwrap()));
            });

            let url = req.uri().to_string();
            let method = req.method().clone();
            let body_bytes = hyper::body::to_bytes(req.body_mut()).await?.to_vec();

            let req = LeafRequest {
                method: method.as_str(),
                uri: url.as_str(),
                headers: &headers,
                body: Some(&body_bytes),
            };

            let resp: LeafResponse = match worker.exports.handle_request(&mut worker.store, req) {
                Ok(r) => r,
                Err(e) => {
                    warn!("---error {:?}", e);
                    return Ok(Response::new(Body::from("Error")));
                }
            };

            let mut builder = Response::builder().status(resp.status);
            for (k, v) in resp.headers {
                builder = builder.header(k, v);
            }
            let resp = builder.body(Body::from(resp.body.unwrap())).unwrap();

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
