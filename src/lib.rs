pub mod app;
pub mod clients;
pub mod common;
pub mod configuration;
pub mod error;
pub mod features;
pub mod middlewares;
pub mod telemetry;

pub use error::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    data: T,
}
