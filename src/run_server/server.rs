use futures::future::{self, Ready};
use hyper::{
    body::Body, http::StatusCode, server::conn::AddrStream, service::Service, Request, Response,
    Server,
};
use lazy_static::lazy_static;
use leaf_host_impl::http::{Request as LeafRequest, Response as LeafResponse};
use leaf_worker::{Manager, Pool};
use mini_moka::sync::Cache;
use std::net::SocketAddr;
use std::path::Path;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::{Duration, Instant};
use tracing::log::debug;
use tracing::{info, warn};

lazy_static! {
    pub static ref WORKERS_CACHE: Cache<String, Pool> = Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60))
        .time_to_idle(Duration::from_secs(10 * 60))
        .build();
}

struct RequestContext {
    pub wasm_dir: String,
}

impl Service<Request<Body>> for RequestContext {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let st = Instant::now();

        // get wasm path from header
        let wasm_name = match req.headers().get("x-wasm-path") {
            Some(path) => path.to_str().unwrap(),
            None => {
                let resp = create_response(
                    StatusCode::BAD_REQUEST,
                    "header x-wasm-path is required".to_string(),
                );
                let fut = async move {
                    info!(
                        "[Request]  {} {} {} {}ms",
                        req.method(),
                        req.uri(),
                        resp.status(),
                        st.elapsed().as_millis()
                    );
                    Ok(resp)
                };
                return Box::pin(fut);
            }
        };
        let wasm_key = wasm_name.to_string();

        let wasm_path = Path::new(&self.wasm_dir).join(wasm_name);
        if !wasm_path.exists() {
            let resp = create_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "wasm file not found".to_string(),
            );
            let fut = async move {
                info!(
                    "[Request] {} {} {} {}ms",
                    req.method(),
                    req.uri(),
                    resp.status(),
                    st.elapsed().as_millis()
                );
                Ok(resp)
            };
            return Box::pin(fut);
        }

        debug!("[Request] wasm path: {:?}", wasm_path);

        let fut = async move {
            // get pool
            let pool = match WORKERS_CACHE.get(&wasm_key) {
                Some(pool) => pool,
                None => {
                    info!(
                        "[Worker] create new pool for wasm: {}",
                        wasm_path.to_str().unwrap()
                    );
                    let mgr = Manager::new(wasm_path.to_str().unwrap().to_string(), false);
                    let pool = Pool::builder(mgr).build().unwrap();
                    WORKERS_CACHE.insert(wasm_key, pool.clone());
                    pool
                }
            };

            // get worker
            let mut worker = pool.get().await.unwrap();
            let worker = worker.as_mut();

            // call worker
            let mut headers: Vec<(&str, &str)> = vec![];
            let req_headers = req.headers().clone();
            req_headers.iter().for_each(|(k, v)| {
                headers.push((k.as_str(), v.to_str().unwrap()));
            });

            let url = req.uri().to_string();
            let method = req.method().clone();
            let body_bytes = hyper::body::to_bytes(req.body_mut()).await?.to_vec();

            let leaf_req = LeafRequest {
                request_id: 1,
                method: method.as_str(),
                uri: url.as_str(),
                headers: &headers,
                body: Some(&body_bytes),
            };

            let resp: LeafResponse = match worker.handle_request(leaf_req).await {
                Ok(r) => r,
                Err(e) => {
                    warn!(
                        "[Request] {} {}, execute failed: {:?}, {}ms",
                        req.method(),
                        req.uri(),
                        e,
                        st.elapsed().as_millis()
                    );
                    return Ok(create_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    ));
                }
            };

            let mut builder = Response::builder().status(resp.status);
            for (k, v) in resp.headers {
                builder = builder.header(k, v);
            }
            let resp = builder.body(Body::from(resp.body.unwrap())).unwrap();

            info!(
                "[Request] {} {} {} {}ms",
                req.method(),
                req.uri(),
                resp.status(),
                st.elapsed().as_millis()
            );
            Ok(resp)
        };
        Box::pin(fut)
    }
}

struct ServerContext {
    pub config: super::RunnerConfig,
}

impl<'addr> Service<&'addr AddrStream> for ServerContext {
    type Response = RequestContext;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _addr: &'addr AddrStream) -> Self::Future {
        future::ok(RequestContext {
            wasm_dir: self.config.wasm_dir.clone(),
        })
    }
}

pub async fn start(addr: SocketAddr, config: super::RunnerConfig) {
    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(ServerContext { config }),
        Err(e) => {
            panic!("[Server] starting failed to bind: {}", e);
        }
    };

    info!("[Server] starting on {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        panic!("[Server] starting error: {}", e);
    }
}

pub fn create_response(status: StatusCode, message: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(message))
        .unwrap()
}
