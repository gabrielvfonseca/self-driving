//! Data models for infrastructure resources.

use serde::{Deserialize, Serialize};

/// Resource type enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    /// Compute instance
    Compute,
    /// Storage volume
    Storage,
    /// Network resource
    Network,
    /// Database instance
    Database,
    /// Load balancer
    LoadBalancer,
    /// Object storage
    ObjectStorage,
}

/// Resource status enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ResourceStatus {
    /// Resource is pending creation
    Pending,
    /// Resource is ready for use
    Ready,
    /// Resource is being updated
    Updating,
    /// Resource is being deleted
    Deleting,
    /// Resource has failed
    Failed,
}

/// Base resource structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Unique identifier for the resource
    pub id: String,
    /// Name of the resource
    pub name: String,
    /// Type of the resource
    pub resource_type: ResourceType,
    /// Current status of the resource
    pub status: ResourceStatus,
    /// Provider identifier (aws, gcp, azure)
    pub provider: String,
    /// Resource metadata
    pub metadata: ResourceMetadata,
    /// Creation timestamp
    pub created_at: i64,
    /// Last updated timestamp
    pub updated_at: i64,
}

/// Resource metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Resource specifications
    pub specs: ResourceSpecs,
    /// Resource labels
    pub labels: std::collections::HashMap<String, String>,
    /// Resource annotations
    pub annotations: std::collections::HashMap<String, String>,
}

/// Resource specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpecs {
    /// CPU cores
    pub cpu: Option<u32>,
    /// Memory in GB
    pub memory_gb: Option<u32>,
    /// Storage in GB
    pub storage_gb: Option<u32>,
    /// Network bandwidth in Mbps
    pub bandwidth_mbps: Option<u32>,
    /// Region identifier
    pub region: Option<String>,
    /// Availability zone
    pub availability_zone: Option<String>,
    /// Additional provider-specific specs
    pub provider_specs: std::collections::HashMap<String, String>,
}

/// Resource request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    /// Name of the resource to create
    pub name: String,
    /// Type of resource to create
    pub resource_type: ResourceType,
    /// Resource specifications
    pub specs: ResourceSpecs,
    /// Provider to use (optional)
    pub provider: Option<String>,
    /// Labels to apply
    pub labels: std::collections::HashMap<String, String>,
    /// Annotations to apply
    pub annotations: std::collections::HashMap<String, String>,
}

/// Resource response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceResponse {
    /// Created resource
    pub resource: Resource,
    /// Operation status
    pub status: ResourceStatus,
    /// Error message if operation failed
    pub error: Option<String>,
}
