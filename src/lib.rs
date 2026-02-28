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

use crate::features::shared::Metadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaginationMetadata {
    total_count: usize,
    total_pages: usize,
    current_page: usize,
    has_previous: bool,
    has_next: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    metadata: PaginationMetadata,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new<U>(data: Vec<U>, value: Metadata, converter: impl FnOnce(Vec<U>) -> Vec<T>) -> Self {
        let (total_count, total_pages, current_page, has_previous, has_next) = value.into_inner();

        Self {
            data: converter(data),
            metadata: PaginationMetadata {
                total_count,
                total_pages,
                current_page,
                has_previous,
                has_next,
            },
        }
    }
}
