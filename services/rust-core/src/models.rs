//! Resource data models for the AI-Native Self-Driving Infrastructure Platform

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Compute resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResource {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub region: String,
    pub instance_type: String,
    pub status: ResourceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: std::collections::HashMap<String, String>,
    pub metadata: std::collections::HashMap<String, String>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: std::collections::HashMap<String, String>,
    pub metadata: std::collections::HashMap<String, String>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: std::collections::HashMap<String, String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Resource status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Resource type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    Compute,
    Storage,
    Network,
}

/// Resource specification for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub resource_type: ResourceType,
    pub provider: String,
    pub region: String,
    pub name: String,
    pub attributes: std::collections::HashMap<String, String>,
    pub tags: std::collections::HashMap<String, String>,
}

/// Resource operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceResult {
    pub resource_id: String,
    pub status: ResourceStatus,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

/// Resource query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuery {
    pub resource_type: Option<ResourceType>,
    pub provider: Option<String>,
    pub region: Option<String>,
    pub status: Option<ResourceStatus>,
    pub tags: std::collections::HashMap<String, String>,
    pub limit: u32,
    pub offset: u32,
}

impl Default for ResourceQuery {
    fn default() -> Self {
        Self {
            resource_type: None,
            provider: None,
            region: None,
            status: None,
            tags: std::collections::HashMap::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl ComputeResource {
    pub fn new(
        id: String,
        name: String,
        provider: String,
        region: String,
        instance_type: String,
    ) -> Self {
        Self {
            id,
            name,
            provider,
            region,
            instance_type,
            status: ResourceStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl StorageResource {
    pub fn new(
        id: String,
        name: String,
        provider: String,
        region: String,
        storage_type: String,
        size_gb: u64,
    ) -> Self {
        Self {
            id,
            name,
            provider,
            region,
            storage_type,
            size_gb,
            status: ResourceStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl NetworkResource {
    pub fn new(
        id: String,
        name: String,
        provider: String,
        region: String,
        network_type: String,
        cidr_block: String,
    ) -> Self {
        Self {
            id,
            name,
            provider,
            region,
            network_type,
            cidr_block,
            status: ResourceStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }
}
