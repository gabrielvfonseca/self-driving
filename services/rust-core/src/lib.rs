//! Core infrastructure library for the AI-Native Self-Driving Infrastructure Platform

pub mod models;
pub mod grpc;

/// Core resource types
pub mod resource_types {
    use serde::{Deserialize, Serialize};
    
    /// Compute resource definition
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ComputeResource {
        pub id: String,
        pub name: String,
        pub provider: String,
        pub region: String,
        pub instance_type: String,
        pub status: ResourceStatus,
        pub created_at: chrono::DateTime<chrono::Utc>,
    }
    
    /// Storage resource definition
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StorageResource {
        pub id: String,
        pub name: String,
        pub provider: String,
        pub region: String,
        pub storage_type: String,
        pub size_gb: u64,
        pub status: ResourceStatus,
        pub created_at: chrono::DateTime<chrono::Utc>,
    }
    
    /// Network resource definition
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NetworkResource {
        pub id: String,
        pub name: String,
        pub provider: String,
        pub region: String,
        pub network_type: String,
        pub cidr_block: String,
        pub status: ResourceStatus,
        pub created_at: chrono::DateTime<chrono::Utc>,
    }
    
    /// Resource status enumeration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ResourceStatus {
        Pending,
        Creating,
        Running,
        Stopping,
        Stopped,
        Terminating,
        Terminated,
        Error,
    }
}

/// Error types for the core library
pub mod errors {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum CoreError {
        #[error("Resource not found: {resource_id}")]
        ResourceNotFound { resource_id: String },
        
        #[error("Invalid resource state: {resource_id}, expected {expected}, got {actual}")]
        InvalidResourceState {
            resource_id: String,
            expected: String,
            actual: String,
        },
        
        #[error("Provider error: {message}")]
        ProviderError { message: String },
    }
}
