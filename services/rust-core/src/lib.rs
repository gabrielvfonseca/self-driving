//! Core infrastructure engine for AI-native self-driving platform
//!
//! This crate provides the foundational data models and core functionality
//! for managing cloud resources across multiple providers.

pub mod models;
pub mod resources;
pub mod grpc;

/// Core error type for the infrastructure engine
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Invalid resource specification: {0}")]
    InvalidResource(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}