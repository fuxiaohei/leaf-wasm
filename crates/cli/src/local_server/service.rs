use futures::future::{self, Ready};
use hyper::{
    body::Body, http::StatusCode, server::conn::AddrStream, service::Service, Request, Response,
};
use leaf_common::errors::Error;
use leaf_host_impl::http::{Request as LeafRequest, Response as LeafResponse};
use leaf_worker::{Manager, Pool};
use std::sync::{atomic::AtomicU64, Arc};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Instant;
use tracing::{info, warn};

pub struct ServiceContext {
    req_id: Arc<AtomicU64>,
    worker_pool: Arc<Pool>,
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
        let worker_pool = self.worker_pool.clone();
        let fut = async move {
            let st = Instant::now();
            let mut worker = match worker_pool.get().await {
                Ok(pool) => pool,
                Err(e) => {
                    warn!(
                        "[Request] id={} {} {}, get from pool failed: {:?}, {}ms",
                        req_id,
                        req.method(),
                        req.uri(),
                        e,
                        st.elapsed().as_millis()
                    );
                    return Ok(create_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    ));
                }
            };
            let worker = worker.as_mut();

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
                request_id: req_id,
                method: method.as_str(),
                uri: url.as_str(),
                headers: &headers,
                body: Some(&body_bytes),
            };

            let resp: LeafResponse = match worker.handle_request(leaf_req).await {
                Ok(r) => r,
                Err(e) => {
                    warn!(
                        "[Request] id={} {} {}, execute failed: {:?}, {}ms",
                        req_id,
                        req.method(),
                        req.uri(),
                        e,
                        st.elapsed().as_millis()
                    );
                    return Ok(create_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    ));
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
    req_id: Arc<AtomicU64>,
    worker_pool: Arc<Pool>,
}

impl ServerContext {
    pub fn new(wasm_file: String, enable_wasi: bool) -> Result<Self, Error> {
        let mgr = Manager::new(wasm_file, enable_wasi);
        let pool = Pool::builder(mgr)
            .build()
            .map_err(|e| Error::InitComponentManagerPool(anyhow::anyhow!(e)))?;
        Ok(Self {
            req_id: Arc::new(AtomicU64::new(1)),
            worker_pool: Arc::new(pool),
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
        let worker_pool = self.worker_pool.clone();
        future::ok(ServiceContext {
            req_id,
            worker_pool,
        })
    }
}
