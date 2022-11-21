use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};
use log::{error, info};
use std::net::SocketAddr;

async fn handler(_req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Hello, World")))
}

pub async fn start(addr: SocketAddr) {
    info!("Starting server on {}", addr);

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_service = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Error>(service_fn(handler))
    });

    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(make_service),
        Err(e) => {
            error!("Starting server error, failed to bind: {}", e);
            return;
        }
    };

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("Starting server error: {}", e);
    }
}
