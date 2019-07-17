use std::fmt::Debug;
use crate::store::{Store, StoreError};
use crate::Eventific;
use grpcio::{RpcContext, UnarySink, RpcStatus, RpcStatusCode};
use failure::Error;
use uuid::Uuid;
use slog::Logger;
use futures::{Future, IntoFuture};
use crate::eventific::EventificError;
use crate::aggregate::Aggregate;

pub fn grpc_command_new_aggregate<
    S: 'static + Default,
    D: 'static + Send + Sync + Debug + Clone,
    St: Store<D>,
    Req: 'static,
    Resp: 'static,
    IC: FnOnce(&Req) -> &str,
    VC: FnOnce(&Req) -> Result<Vec<D>, Error>,
    RC: 'static + FnOnce() -> Resp + Send
>(
    eventific: &Eventific<S, D, St>,
    ctx: RpcContext,
    req: Req,
    resp: UnarySink<Resp>,
    id_callback: IC,
    event_callback: VC,
    result_callback: RC
) {
    let logger = eventific.get_logger();
    handle_uuid(logger, &req, resp, id_callback, |uuid, sink| {
        let log = logger.new(o!("aggregate_id" => uuid.to_string()));
        match event_callback(&req) {
            Ok(events) => {
                let create_fut = eventific.create_aggregate(uuid, events, None)
                    .then(move |res| {
                        match res {
                            Ok(_) => {
                                let err_logger = log.clone();
                                let result = result_callback();
                                let res_fut = sink.success(result)
                                    .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                                tokio::spawn(res_fut);
                                Ok(())
                            },
                            Err(err) => {
                                match err {
                                    EventificError::StoreError(s_err) => {
                                        match s_err {
                                            StoreError::EventAlreadyExists(_) => {
                                                let err_logger = log.new(o!("internal_error" => format!("{}", s_err)));
                                                let status = RpcStatus::new(RpcStatusCode::AlreadyExists, Some("The aggregate does already exist".to_owned()));
                                                let res_fut = sink.fail(status)
                                                    .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                                                tokio::spawn(res_fut);
                                                Ok(())
                                            },
                                            StoreError::Unknown(e_err) => {
                                                let err_logger = log.new(o!("internal_error" => format!("{}", e_err)));
                                                warn!(err_logger, "Internal error occurred");
                                                let status = RpcStatus::new(RpcStatusCode::Internal, None);
                                                let res_fut = sink.fail(status)
                                                    .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                                                tokio::spawn(res_fut);
                                                Ok(())
                                            },
                                        }
                                    },
                                    EventificError::SendNotificationError(_) => {Ok(())},
                                    EventificError::SendNotificationInitError(_) => {Ok(())},
                                    EventificError::ListenNotificationError(_) => {Ok(())},
                                    EventificError::ListenNotificationInitError(_) => {Ok(())},
                                    _ => {
                                        let err_logger = log.new(o!("internal_error" => format!("{}", err)));
                                        warn!(err_logger, "Internal error occurred");
                                        let status = RpcStatus::new(RpcStatusCode::Internal, None);
                                        let res_fut = sink.fail(status)
                                            .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                                        tokio::spawn(res_fut);
                                        Ok(())
                                    }
                                }
                            },
                        }
                    });
                eventific.spawn(create_fut);
            },
            Err(err) => {
                let err_logger = log.new(o!("validation_error" => format!("{}", err)));
                let status = RpcStatus::new(RpcStatusCode::InvalidArgument, Some(format!("{}", err)));
                let res_fut = sink.fail(status)
                    .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                eventific.spawn(res_fut);
            },
        }
    })
}

pub fn grpc_command_existing_aggregate<
    S: 'static + Default + Send,
    D: 'static + Send + Sync + Debug + Clone,
    St: Store<D> + Sync,
    Req: 'static + Sync + Send + Clone,
    Resp: 'static,
    IC: FnOnce(&Req) -> &str,
    VC: 'static + Fn(&Req, Aggregate<S>) -> IF + Send,
    RC: 'static + FnOnce() -> Resp + Send,
    IF: 'static + IntoFuture<Item=Vec<D>, Error=Error, Future=FF>,
    FF: 'static + Future<Item=Vec<D>, Error=Error> + Send
>(
    eventific: &Eventific<S, D, St>,
    ctx: RpcContext,
    req: Req,
    resp: UnarySink<Resp>,
    id_callback: IC,
    event_callback: VC,
    result_callback: RC
) {
    let logger = eventific.get_logger();
    let eve = eventific.clone();
    handle_uuid(logger, &req.clone(), resp, id_callback, move |uuid, sink| {
        let log = logger.new(o!("aggregate_id" => uuid.to_string()));
        let add_future = eventific.add_events_to_aggregate(uuid, None, move |aggregate| {
            event_callback(&req, aggregate)
        })
            .then(move |res| {
                match res {
                    Ok(_) => {
                        let err_logger = log.clone();
                        let result = result_callback();
                        let res_fut = sink.success(result)
                            .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                        tokio::spawn(res_fut);
                        Ok(())
                    },
                    Err(err) => {
                        match err {
                            EventificError::ValidationError(v_err) => {
                                let err_logger = log.new(o!("validation_error" => format!("{}", v_err)));
                                let status = RpcStatus::new(RpcStatusCode::InvalidArgument, Some(format!("{}", v_err)));
                                let res_fut = sink.fail(status)
                                    .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                                tokio::spawn(res_fut);
                                Ok(())
                            },
                            _ => {
                                let err_logger = log.new(o!("internal_error" => format!("{}", err)));
                                warn!(err_logger, "Internal error occurred");
                                let status = RpcStatus::new(RpcStatusCode::Internal, None);
                                let res_fut = sink.fail(status)
                                    .map_err(move |e| error!(err_logger, "Failed to send response"; "grpc_error" => format!("{}", e)));
                                tokio::spawn(res_fut);
                                Ok(())
                            }
                        }
                    },
                }
            });
        eve.spawn(add_future);
    })
}

fn handle_uuid<Req, Resp, IC: FnOnce(&Req) -> &str, CC: FnOnce(Uuid, UnarySink<Resp>)>(logger: &Logger, req: &Req, resp: UnarySink<Resp>, id_callback: IC, callback: CC) {
    let raw_id = id_callback(req);
    let log = logger.new(o!("aggregate_id" => raw_id.to_owned()));
    match Uuid::parse_str(raw_id) {
        Ok(uuid) => {
            callback(uuid, resp);
        },
        Err(err) => {
            let err_log = log.new(o!("validation_error" => format!("{}", err)));
            info!(err_log, "Provided aggregate id was not a valid UUID");
            let status = RpcStatus::new(RpcStatusCode::InvalidArgument, Some("Provided aggregate id was not a valid UUID".to_owned()));
            let resp_fut = resp.fail(status)
                .map_err(move |e| error!(err_log, "Failed to send response"; "grpc_error" => format!("{}", e)));

            tokio::spawn(resp_fut);
        },
    }
}
