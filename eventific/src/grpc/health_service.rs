use crate::grpc::health_grpc::Health;
use grpc::{RequestOptions, SingleResponse};
use crate::grpc::health::{HealthCheckRequest, HealthCheckResponse, HealthCheckResponse_ServingStatus};

#[derive(Default)]
pub struct HealthService {

}

impl Health for HealthService {
    fn check(&self, _o: RequestOptions, _p: HealthCheckRequest) -> SingleResponse<HealthCheckResponse> {
        SingleResponse::no_metadata(futures::finished(HealthCheckResponse {
            status: HealthCheckResponse_ServingStatus::SERVING,
            unknown_fields: Default::default(),
            cached_size: Default::default()
        }))
    }
}
