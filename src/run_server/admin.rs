use futures::future::{self, Ready};
use hyper::{
    body::Body, http::StatusCode, server::conn::AddrStream, service::Service, Request, Response,
    Server,
};
use std::net::SocketAddr;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::info;
struct AdminReqCtx {}

impl Service<Request<Body>> for AdminReqCtx {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let resp = super::create_response(StatusCode::OK, "Hello Admin".to_string());
        let fut = async move { Ok(resp) };
        Box::pin(fut)
    }
}

struct AdminSvrCtx {}

impl<'addr> Service<&'addr AddrStream> for AdminSvrCtx {
    type Response = AdminReqCtx;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _addr: &'addr AddrStream) -> Self::Future {
        future::ok(AdminReqCtx {})
    }
}

pub async fn start_admin(addr: SocketAddr, _config: super::RunnerConfig) {
    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(AdminSvrCtx {}),
        Err(e) => {
            panic!("[adminSvr] starting failed to bind: {}", e);
        }
    };

    info!("[adminSvr] starting on {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        panic!("[adminSvr] starting error: {}", e);
    }
}
