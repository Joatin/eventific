use slog::Logger;

#[derive(Clone)]
pub struct StoreContext {
    pub logger: Logger,
    pub service_name: String,
}

impl StoreContext {
    pub fn logger(&self) -> &Logger {
        &self.logger
    }
}
