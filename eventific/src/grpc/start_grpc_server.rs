use slog::Logger;
use crate::Eventific;
use std::fmt::Debug;
use crate::store::Store;
use std::mem::forget;
use crate::eventific::EventificError;
use grpc::rt::ServerServiceDefinition;
use grpc::ServerBuilder;
use crate::grpc::health_grpc::HealthServer;
use crate::grpc::health_service::HealthService;

pub(crate) fn start_grpc_server<S, D: 'static + Send + Sync + Debug, St: Store<D>>(
    logger: &Logger,
    eventific: Eventific<S, D, St>,
    addr: &str,
    grpc_services: Vec<Box<dyn Fn(Eventific<S, D, St>) -> ServerServiceDefinition + Send>>
) -> Result<(), EventificError<D>> {
    let mut builder = ServerBuilder::<tls_api_stub::TlsAcceptor>::new();
    builder.http.set_addr(&addr).expect("The grpc address has to be valid");

    for callback in grpc_services {
        builder.add_service(callback(eventific.clone()));
    }

    builder.add_service(HealthServer::new_service_def(HealthService::default()));

    let server = builder
        .build()
        .map_err(|err|EventificError::InitError(format_err!("{}", err)))?;

    info!(logger, "Started GRPC server at http://{}", addr);

    // prevent drop from being called when this scope goes out
    forget(server);

    // Keep tokio running forever
    tokio::spawn(futures::empty());

    Ok(())
}
