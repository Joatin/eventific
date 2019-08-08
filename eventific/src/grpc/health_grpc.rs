// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Health {
    fn check(&self, o: ::grpc::RequestOptions, p: super::health::HealthCheckRequest) -> ::grpc::SingleResponse<super::health::HealthCheckResponse>;
}

// client

pub struct HealthClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Check: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::health::HealthCheckRequest, super::health::HealthCheckResponse>>,
}

impl ::grpc::ClientStub for HealthClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HealthClient {
            grpc_client: grpc_client,
            method_Check: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grpc.health.v1.Health/Check".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Health for HealthClient {
    fn check(&self, o: ::grpc::RequestOptions, p: super::health::HealthCheckRequest) -> ::grpc::SingleResponse<super::health::HealthCheckResponse> {
        self.grpc_client.call_unary(o, p, self.method_Check.clone())
    }
}

// server

pub struct HealthServer;


impl HealthServer {
    pub fn new_service_def<H : Health + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/grpc.health.v1.Health",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grpc.health.v1.Health/Check".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.check(o, p))
                    },
                ),
            ],
        )
    }
}
