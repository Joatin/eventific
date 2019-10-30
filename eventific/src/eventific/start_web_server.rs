use futures::{Future, future, lazy};
use std::{process};
use slog::Logger;
use hyper::service::service_fn;
use hyper::{Method, Response, Body, StatusCode};
use prometheus::{TextEncoder, Encoder};
use hyper::header::{HeaderValue};
use hyper::Server;
use std::net::SocketAddr;


pub fn start_web_server(logger: &Logger, addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    let logger = logger.clone();
    let log = logger.clone();
    let addr = addr.clone();
    let new_service = move || {
        let logger = logger.clone();

        service_fn(move |req| -> Box<dyn Future<Item = _, Error = hyper::Error> + Send> {
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/metrics") => {
                    debug!(logger, "Metrics collected!");
                    let encoder = TextEncoder::new();
                    let metric_families = prometheus::gather();
                    let mut buffer = vec![];
                    encoder.encode(&metric_families, &mut buffer).unwrap();
                    let mut response = Response::new(Body::from(buffer));
                    response.headers_mut().insert("Content-Type", HeaderValue::from_str(&encoder.format_type().to_string()).unwrap());
                    *response.status_mut() = StatusCode::OK;
                    Box::new(future::ok(response))
                }
                (&Method::GET, "/health") => {
                    debug!(logger, "Health check!");
                    let mut response = Response::new(Body::from("\"OK\""));
                    *response.status_mut() = StatusCode::OK;
                    Box::new(future::ok(response))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };

    lazy(move || {
        info!(log, "Web server listening on http://{}", addr);
        info!(log, "You can get prometheus metrics at http://{}/metrics", addr);
        Server::bind(&addr)
            .serve(new_service)
            .map_err(move |err| {
                eprintln!("Eventific web server failed!");
                eprintln!("Internal Error: {}", format!("{}", err));
                process::exit(1)
            })
    })
}
