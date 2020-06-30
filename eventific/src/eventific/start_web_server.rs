use std::{process};
use slog::Logger;
use hyper::service::{service_fn, make_service_fn};
use hyper::{Method, Response, Body, StatusCode, Server, Request};
use prometheus::{TextEncoder, Encoder};
use hyper::header::{HeaderValue};
use std::net::SocketAddr;
use uuid::Uuid;
use failure::Error;


pub async fn start_web_server(logger: Logger, addr: SocketAddr) {
    info!(logger, "Starting web server");

    let logger = logger.clone();
    let log = logger.clone();
    let addr = addr.clone();


    let new_service = make_service_fn(move |_| {
        let logger = logger.clone();

        async move {
            let logger = logger.clone();
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let logger = logger.new(o!("request_id" => Uuid::new_v4().to_string()));

                info!(logger, "Received request"; "method" => req.method().to_string(), "path" => req.uri().path().to_string());

                handle_request(logger.clone(), req)
            }))
        }
    });

    let server = Server::bind(&addr)
        .serve(new_service)
        .with_graceful_shutdown(shutdown_signal(&log));

    info!(log, "Server started! 🚀"; "address" => format!("http://{}", addr));
    info!(log, "You can get prometheus metrics at http://{}/metrics", addr);
    info!(log, "You can get health status at http://{}/health", addr);

    if let Err(e) = server.await {
        eprintln!("Eventific web server failed!");
        eprintln!("Internal Error: {}", format!("{}", e));
        process::exit(1)
    }
}

async fn shutdown_signal(logger: &Logger) {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    info!(logger, "Shutting down 👋");
}

async fn handle_request(logger: Logger, req: Request<Body>) -> Result<Response<Body>, Error> {
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
            Ok(response)
        }
        (&Method::GET, "/health") => {
            debug!(logger, "Health check!");
            let mut response = Response::new(Body::from("\"OK\""));
            *response.status_mut() = StatusCode::OK;
            Ok(response)
        }
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

