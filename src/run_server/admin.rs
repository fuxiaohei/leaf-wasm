use futures::future::{self, Ready};
use hyper::{
    body::Body, http::StatusCode, server::conn::AddrStream, service::Service, Request, Response,
    Server,
};
use leaf_worker::{Manager, Pool};
use matchit::Router;
use std::net::SocketAddr;
use std::path::Path;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::{info, warn};

fn reload_wasm(req: Request<Body>, wasm_dir: String) -> Response<Body> {
    let query_params = super::query::query_decode(req.uri().query().unwrap_or_default());
    let wasm_file = match super::query::query_get(&query_params, "file") {
        Some(f) => f,
        None => {
            let resp =
                super::create_response(StatusCode::BAD_REQUEST, "file is required".to_string());
            return resp;
        }
    }
    .to_string();
    if super::WORKERS_CACHE.contains_key(&wasm_file) {
        info!("[admin] invalidate cache: {:?}", wasm_file);
        super::WORKERS_CACHE.invalidate(&wasm_file);
    }
    let wasm_path = Path::new(&wasm_dir).join(&wasm_file);
    if !wasm_path.exists() {
        warn!("[admin] reload file not found: {wasm_path:?}");
        let resp = super::create_response(
            StatusCode::BAD_REQUEST,
            format!("file not found: {wasm_path:?}"),
        );
        return resp;
    }
    let mgr = Manager::new(wasm_path.to_str().unwrap().to_string(), false);
    let pool = Pool::builder(mgr).build().unwrap();
    let wasm_key = wasm_file.clone();

    super::WORKERS_CACHE.insert(wasm_key, pool);
    info!("[admin] reload wasm ok: {wasm_file:?}");

    super::create_response(StatusCode::OK, "ok".to_string())
}

struct AdminReqCtx {
    pub router: Router<String>,
    pub wasm_dir: String,
}

impl Service<Request<Body>> for AdminReqCtx {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // match simple path
        let matched = match self.router.at(req.uri().path()) {
            Ok(m) => m,
            Err(e) => {
                warn!("[admin] request path {}, error: {}", req.uri().path(), e);
                let resp = super::create_response(StatusCode::NOT_FOUND, "Not Found".to_string());
                let fut = async move { Ok(resp) };
                return Box::pin(fut);
            }
        };
        let resp = match matched.value.as_str() {
            "reload" => reload_wasm(req, self.wasm_dir.clone()),
            _ => super::create_response(StatusCode::OK, "Hello Admin".to_string()),
        };
        let fut = async move { Ok(resp) };
        Box::pin(fut)
    }
}

struct AdminSvrCtx {
    pub router: Router<String>,
    pub config: super::RunnerConfig,
}

impl AdminSvrCtx {
    pub fn new(config: super::RunnerConfig) -> Self {
        let mut router = Router::new();
        router.insert("/reload", String::from("reload")).unwrap();
        AdminSvrCtx { router, config }
    }
}

impl<'addr> Service<&'addr AddrStream> for AdminSvrCtx {
    type Response = AdminReqCtx;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _addr: &'addr AddrStream) -> Self::Future {
        future::ok(AdminReqCtx {
            router: self.router.clone(),
            wasm_dir: self.config.wasm_dir.clone(),
        })
    }
}

pub async fn start_admin(addr: SocketAddr, config: super::RunnerConfig) {
    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(AdminSvrCtx::new(config)),
        Err(e) => {
            panic!("[admin] starting failed to bind: {e}");
        }
    };

    info!("[admin] starting on {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        panic!("[admin] starting error: {e}");
    }
}
