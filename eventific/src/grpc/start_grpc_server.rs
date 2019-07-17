use slog::Logger;
use crate::Eventific;
use std::fmt::Debug;
use crate::store::Store;
use std::sync::Arc;
use std::mem::forget;
use crate::eventific::EventificError;

pub(crate) fn start_grpc_server<S, D: 'static + Send + Sync + Debug, St: Store<D>>(
    logger: &Logger,
    eventific: Eventific<S, D, St>,
    grpc_services: Vec<Box<dyn Fn(Eventific<S, D, St>) -> grpcio::Service + Send>>
) -> Result<(), EventificError<D>> {
    let mut builder = grpcio::ServerBuilder::new(Arc::new(grpcio::Environment::new(4)));

    for callback in grpc_services {
        builder = builder.register_service(callback(eventific.clone()));
    }

    let mut server = builder
        .bind("localhost", 5000)
        .build()
        .map_err(|err|EventificError::InitError(format_err!("{}", err)))?;

    info!(logger, "Starting GRPC server at http://localhost:5000");
    server.start();

    // prevent drop from being called when this scope goes out
    forget(server);

    // Keep tokio running forever
    tokio::spawn(futures::empty());

    Ok(())
}
