use slog::Logger;
use crate::Eventific;
use std::fmt::Debug;
use crate::store::Store;
use std::sync::Arc;
use std::mem::forget;
use crate::eventific::EventificError;
use grpc::rt::ServerServiceDefinition;
use grpc::ServerBuilder;

pub(crate) fn start_grpc_server<S, D: 'static + Send + Sync + Debug, St: Store<D>>(
    logger: &Logger,
    eventific: Eventific<S, D, St>,
    port: u16,
    grpc_services: Vec<Box<dyn Fn(Eventific<S, D, St>) -> ServerServiceDefinition + Send>>
) -> Result<(), EventificError<D>> {
    let mut builder = ServerBuilder::<tls_api_stub::TlsAcceptor>::new();
    builder.http.set_port(port);

    for callback in grpc_services {
        builder.add_service(callback(eventific.clone()));
    }

    let mut server = builder
        .build()
        .map_err(|err|EventificError::InitError(format_err!("{}", err)))?;

    info!(logger, "Started GRPC server at http://localhost:{}", port);

    // prevent drop from being called when this scope goes out
    forget(server);

    // Keep tokio running forever
    tokio::spawn(futures::empty());

    Ok(())
}
