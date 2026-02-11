//! Core infrastructure management library for the AI-native platform.

pub mod models;
pub mod resources;
pub mod grpc;

pub use models::*;
pub use resources::*;
pub use grpc::*;

/// Main entry point for the infrastructure platform
pub struct InfraPlatform {
    /// Resource manager for infrastructure provisioning
    pub resource_manager: ResourceManager,
}

impl InfraPlatform {
    /// Create a new infrastructure platform instance
    pub fn new() -> Self {
        Self {
            resource_manager: ResourceManager::new(),
        }
    }
}
