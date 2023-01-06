use crate::common::errors::Error;
use crate::wasm::{PreManager, PrePool};
use crate::wit::{LeafHttp, Request as LeafRequest, Response as LeafResponse};
use futures::future::{self, Ready};
use hyper::{
    body::Body, http::StatusCode, server::conn::AddrStream, service::Service, Request, Response,
};
use log::{info, warn};
use once_cell::sync::OnceCell;
use std::sync::{atomic::AtomicU64, Arc};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Instant;
use wasmtime::Store;

static PRE_POOL: OnceCell<PrePool> = OnceCell::new();

pub struct ServiceContext {
    pub req_id: Arc<AtomicU64>,
}

impl Service<Request<Body>> for ServiceContext {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let req_id = self
            .req_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let fut = async move {
            let st = Instant::now();
            let pool = PRE_POOL.get().unwrap();
            let worker = match pool.get().await {
                Ok(pool) => pool,
                Err(e) => {
                    warn!(
                        "[Request] id={} {} {}, get from pool failed: {:?}, {:?}",
                        req_id,
                        req.method(),
                        req.uri(),
                        e,
                        st.elapsed()
                    );
                    return Ok(create_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    ));
                }
            };
            let mut context = crate::wasm::Context::new();
            context.fetch().req_id = req_id;
            let mut store = Store::new(&worker.engine, context);
            let instance = worker
                .instance_pre
                .instantiate_async(&mut store)
                .await
                .unwrap();
            let exports = LeafHttp::new(&mut store, &instance).unwrap();

            // convert hyper Request to wasm request
            let mut headers: Vec<(&str, &str)> = vec![];
            let req_headers = req.headers().clone();
            req_headers.iter().for_each(|(k, v)| {
                headers.push((k.as_str(), v.to_str().unwrap()));
            });

            let url = req.uri().to_string();
            let method = req.method().clone();
            let body_bytes = hyper::body::to_bytes(req.body_mut()).await?.to_vec();

            let leaf_req = LeafRequest {
                method: method.as_str(),
                uri: url.as_str(),
                headers: &headers,
                body: Some(&body_bytes),
            };

            let resp: LeafResponse = match exports.handle_request(&mut store, leaf_req).await {
                Ok(r) => r,
                Err(e) => {
                    warn!(
                        "[Request] id={} {} {}, execute failed: {:?}, {:?}",
                        req_id,
                        req.method(),
                        req.uri(),
                        e,
                        st.elapsed()
                    );
                    return Ok(Response::new(Body::from(format!("Error : {}", e))));
                }
            };

            // convert wasm response to hyper response
            let mut builder = Response::builder().status(resp.status);
            for (k, v) in resp.headers {
                builder = builder.header(k, v);
            }
            let resp = builder.body(Body::from(resp.body.unwrap())).unwrap();

            info!(
                "[Request] id={} {} {} {} {:?}",
                req_id,
                req.method(),
                req.uri(),
                resp.status(),
                st.elapsed()
            );
            Ok(resp)
        };
        Box::pin(fut)
    }
}

fn create_error_response(status: StatusCode, message: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(message))
        .unwrap()
}

pub struct ServerContext {
    pub req_id: Arc<AtomicU64>,
}

impl ServerContext {
    pub fn new(wasm_file: String) -> Result<Self, Error> {
        let pre_mgr = PreManager::new(wasm_file);
        let pre_pool = PrePool::builder(pre_mgr)
            .build()
            .map_err(|e| Error::InitComponentManagerPool(anyhow::anyhow!(e)))?;
        PRE_POOL.set(pre_pool).map_err(|e| {
            Error::InitComponentManagerPool(anyhow::anyhow!("Failed to set pre_pool : {:?}", e))
        })?;

        Ok(Self {
            req_id: Arc::new(AtomicU64::new(1)),
        })
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
        let req_id = self.req_id.clone();
        future::ok(ServiceContext { req_id })
    }
}
