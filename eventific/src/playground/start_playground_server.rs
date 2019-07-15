use slog::Logger;
use crate::Eventific;
use std::fmt::Debug;
use crate::store::Store;
use hyper::{Server, Request, Body, Response};
use hyper::header::HeaderValue;
use futures::future::Future;
use hyper::service::service_fn_ok;
use std::process;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/playground/build/"]
struct Asset;

fn playground_site(req: Request<Body>) -> Response<Body> {
    if req.uri().path().starts_with("/api") {
        Response::new(Body::from("Api"))
    } else {
        match Asset::get(&req.uri().path().replacen("/", "", 1)) {
            Some(data) => {
                let mut response = Response::new(Body::from(data));
                if req.uri().path().ends_with(".css") {
                    response.headers_mut().append("Content-Type", HeaderValue::from_static("text/css"));
                } else if req.uri().path().ends_with(".html") {
                    response.headers_mut().append("Content-Type", HeaderValue::from_static("text/html"));
                } else if req.uri().path().ends_with(".js") {
                    response.headers_mut().append("Content-Type", HeaderValue::from_static("text/javascript"));
                } else if req.uri().path().ends_with(".json") {
                    response.headers_mut().append("Content-Type", HeaderValue::from_static("application/json"));
                } else if req.uri().path().ends_with(".ico") {
                    response.headers_mut().append("Content-Type", HeaderValue::from_static("image/x-icon"));
                }
                response
            },
            None => {
                let mut response = Response::new(Body::from(Asset::get("index.html").unwrap()));
                response.headers_mut().append("Content-Type", HeaderValue::from_static("text/html"));
                response
            }
        }
    }
}

pub(crate) fn start_playground_server<S, D: 'static + Send + Sync + Debug, St: Store<D>>(logger: &Logger, _eventific: &Eventific<S, D, St>) -> impl Future<Item = (), Error = ()> {
    let port = 3000;

    info!(logger, "Starting playground server at http://localhost:{}", port);

    let addr = ([127, 0, 0, 1], port).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(playground_site)
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| {
            eprintln!("Playground server error: {}", e);
            process::exit(1)
        })
        .map(|_|())
}
