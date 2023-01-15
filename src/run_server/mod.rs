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
use tokio::time::Instant;
use tracing::info;

pub async fn start(addr: SocketAddr) {
    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(ServerContext {}),
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

struct RequestContext {}

impl Service<Request<Body>> for RequestContext {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let st = Instant::now();
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Hello, Leaf-Runner!"))
            .unwrap();
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
        Box::pin(fut)
    }
}

struct ServerContext {}

impl<'addr> Service<&'addr AddrStream> for ServerContext {
    type Response = RequestContext;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _addr: &'addr AddrStream) -> Self::Future {
        future::ok(RequestContext {})
    }
}
