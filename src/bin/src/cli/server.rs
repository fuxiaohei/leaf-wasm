use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::{body::Incoming as IncomingBody, service::Service, Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{atomic::AtomicI64, atomic::Ordering, Arc};
use tokio::net::TcpListener;
use tokio::time::Instant;
use tracing::{info, info_span, Instrument};

#[derive(Clone)]
struct Svc {
    /// A counter for the number of requests we've seen.
    req_id: Arc<AtomicI64>,
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        let start_time = Instant::now();
        let req_id = self.req_id.fetch_add(1, Ordering::SeqCst);
        let method = req.method().clone();
        let uri = req.uri().clone();

        let res = match req.uri().path() {
            "/" => mk_response(format!("home! counter = {:?}", req_id)),
            "/posts" => mk_response(format!("posts, of course! counter = {:?}", req_id)),
            "/authors" => mk_response(format!("authors extraordinare! counter = {:?}", req_id)),
            // Return the 404 Not Found for other routes, and don't increment counter.
            _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
        };

        Box::pin(
            async move {
                info!(methos = method.as_str(), uri = uri.path(), elapsed = ?start_time.elapsed());
                res
            }
            .instrument(info_span!("[Req]", req_id = req_id)),
        )
    }
}

pub async fn start(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);

    let svc = Svc {
        req_id: Arc::new(AtomicI64::new(0)),
    };

    loop {
        let (stream, _) = listener.accept().await?;

        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, svc_clone)
                .await
            {
                info!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
