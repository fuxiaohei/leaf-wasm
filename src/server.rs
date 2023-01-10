use futures::future::{self, Ready};
use hyper::StatusCode;
use hyper::{body::Body, server::conn::AddrStream, service::Service, Request, Response, Server};
use std::sync::{atomic::AtomicU64, Arc};
use std::{
    convert::Infallible,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::{error, info};

pub struct RequestCtx {
    pub req_id: Arc<AtomicU64>,
}

impl Service<Request<Body>> for RequestCtx {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut _req: Request<Body>) -> Self::Future {
        let fut = async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("Hello"))
                .unwrap();
            Ok(response)
        };
        Box::pin(fut)
    }
}

pub struct ServerCtx {
    pub req_id: Arc<AtomicU64>,
}

impl ServerCtx {
    pub fn new() -> Self {
        Self {
            req_id: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl<'addr> Service<&'addr AddrStream> for ServerCtx {
    type Response = RequestCtx;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _addr: &'addr AddrStream) -> Self::Future {
        let req_id = self.req_id.clone();
        future::ok(RequestCtx { req_id })
    }
}

pub async fn start(addr: SocketAddr) {
    let svc = ServerCtx::new();

    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(svc),
        Err(e) => {
            error!("[Server] starting failed to bind: {}", e);
            return;
        }
    };

    info!("[Server] starting on {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("[Server] starting error: {}", e);
    }
}
