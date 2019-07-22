use crate::eventific::EventificError;
use std::fmt::Debug;
use crate::store::StoreError;

impl<D: 'static + Send + Sync + Debug> Into<grpc::Error> for EventificError<D> {
    fn into(self) -> grpc::Error {
        match self {
            EventificError::ValidationError(err) => {
                grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                    grpc_status: grpc::GrpcStatus::Argument as _,
                    grpc_message: format!("{}", err)
                })
            },
            EventificError::StoreError(store_err) => {
                match store_err {
                    StoreError::EventAlreadyExists(_) => {
                        grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                            grpc_status: grpc::GrpcStatus::AlreadyExists as _,
                            grpc_message: "Aggregate does already exist".to_owned()
                        })
                    },
                    _ => {
                        grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                            grpc_status: grpc::GrpcStatus::Internal as _,
                            grpc_message: "Internal error".to_owned()
                        })
                    }
                }
            }
            _ => {
                grpc::Error::GrpcMessage(grpc::GrpcMessageError {
                    grpc_status: grpc::GrpcStatus::Internal as _,
                    grpc_message: "Internal error".to_owned()
                })
            }
        }
    }
}
