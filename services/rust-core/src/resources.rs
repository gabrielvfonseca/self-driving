//! Resource management functionality

use crate::models::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Compute instance resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeInstance {
    /// Base resource
    pub resource: Resource,
    /// Instance type
    pub instance_type: String,
    /// AMI ID (for AWS)
    pub ami_id: Option<String>,
    /// Public IP address
    pub public_ip: Option<String>,
    /// Private IP address
    pub private_ip: Option<String>,
    /// SSH key name
    pub ssh_key: Option<String>,
    /// User data
    pub user_data: Option<String>,
    /// CPU cores
    pub cpu_cores: u32,
    /// Memory in GB
    pub memory_gb: u32,
    /// Storage configuration
    pub storage: Vec<StorageSpec>,
}

/// Storage resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResource {
    /// Base resource
    pub resource: Resource,
    /// Storage type
    pub storage_type: StorageType,
    /// Size in GB
    pub size_gb: u64,
    /// Volume type
    pub volume_type: String,
    /// Mount point
    pub mount_point: Option<String>,
    /// File system type
    pub filesystem_type: Option<String>,
    /// Encrypted
    pub encrypted: bool,
}

/// Network resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResource {
    /// Base resource
    pub resource: Resource,
    /// Network type
    pub network_type: NetworkType,
    /// CIDR block
    pub cidr_block: String,
    /// Subnets
    pub subnets: Vec<Subnet>,
    /// Security groups
    pub security_groups: Vec<SecurityGroup>,
    /// Internet gateway
    pub internet_gateway: Option<String>,
    /// Route table
    pub route_table: Option<String>,
}

/// Database resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseResource {
    /// Base resource
    pub resource: Resource,
    /// Database engine
    pub engine: DatabaseEngine,
    /// Engine version
    pub engine_version: String,
    /// Instance class
    pub instance_class: String,
    /// Allocated storage
    pub allocated_storage_gb: u64,
    /// Master username
    pub master_username: String,
    /// Backup retention period
    pub backup_retention_period: u32,
    /// Multi-AZ
    pub multi_az: bool,
    /// Storage type
    pub storage_type: String,
}

/// Load balancer resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerResource {
    /// Base resource
    pub resource: Resource,
    /// Load balancer type
    pub lb_type: LoadBalancerType,
    /// Subnets
    pub subnets: Vec<String>,
    /// Security groups
    pub security_groups: Vec<String>,
    /// Listeners
    pub listeners: Vec<Listener>,
    /// Target groups
    pub target_groups: Vec<TargetGroup>,
    /// Scheme
    pub scheme: String,
}

/// Storage specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpec {
    /// Storage size in GB
    pub size_gb: u64,
    /// Storage type
    pub storage_type: String,
    /// Mount point
    pub mount_point: Option<String>,
    /// File system type
    pub filesystem_type: Option<String>,
    /// Encrypted
    pub encrypted: bool,
}

/// Storage type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    /// EBS volume
    Ebs,
    /// GCP Persistent Disk
    GcpPersistentDisk,
    /// Azure Managed Disk
    AzureManagedDisk,
    /// Object storage
    ObjectStorage,
    /// File storage
    FileStorage,
}

/// Network type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    /// Virtual Private Cloud
    Vpc,
    /// Virtual Network
    Vnet,
    /// Network
    Network,
}

/// Database engine enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseEngine {
    /// Amazon RDS
    Mysql,
    /// Amazon RDS
    Postgresql,
    /// Amazon RDS
    Oracle,
    /// Amazon RDS
    SqlServer,
    /// Google Cloud SQL
    MysqlGcp,
    /// Google Cloud SQL
    PostgresqlGcp,
    /// Azure Database
    MysqlAzure,
    /// Azure Database
    PostgresqlAzure,
}

/// Load balancer type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancerType {
    /// Application Load Balancer
    Application,
    /// Network Load Balancer
    Network,
    /// Classic Load Balancer
    Classic,
}

/// Subnet specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    /// Subnet ID
    pub id: String,
    /// CIDR block
    pub cidr_block: String,
    /// Availability zone
    pub availability_zone: String,
    /// Route table ID
    pub route_table_id: Option<String>,
    /// Is public
    pub is_public: bool,
}

/// Security group specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroup {
    /// Security group ID
    pub id: String,
    /// Security group name
    pub name: String,
    /// Description
    pub description: String,
    /// Rules
    pub rules: Vec<SecurityRule>,
}

/// Security rule specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Rule ID
    pub id: String,
    /// Protocol
    pub protocol: String,
    /// Port range
    pub port_range: String,
    /// Source CIDR
    pub source_cidr: String,
    /// Direction
    pub direction: SecurityRuleDirection,
}

/// Security rule direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityRuleDirection {
    /// Inbound rule
    Inbound,
    /// Outbound rule
    Outbound,
}

/// Listener specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    /// Protocol
    pub protocol: String,
    /// Port
    pub port: u16,
    /// SSL policy
    pub ssl_policy: Option<String>,
    /// Certificates
    pub certificates: Vec<Certificate>,
}

/// Certificate specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// Certificate ARN
    pub arn: String,
    /// Certificate name
    pub name: String,
}

/// Target group specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGroup {
    /// Target group name
    pub name: String,
    /// Protocol
    pub protocol: String,
    /// Port
    pub port: u16,
    /// VPC ID
    pub vpc_id: String,
    /// Health check
    pub health_check: HealthCheck,
}

/// Health check specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Health check path
    pub path: String,
    /// Health check port
    pub port: u16,
    /// Health check interval
    pub interval_seconds: u32,
    /// Health check timeout
    pub timeout_seconds: u32,
    /// Health check healthy threshold
    pub healthy_threshold_count: u32,
    /// Health check unhealthy threshold
    pub unhealthy_threshold_count: u32,
}

/// Resource manager for handling different resource types
pub struct ResourceManager {
    /// Map of all resources by ID
    resources: HashMap<Uuid, Resource>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Create a new resource
    pub fn create_resource(&mut self, resource: Resource) -> Result<Uuid, EngineError> {
        let id = resource.id;
        self.resources.insert(id, resource);
        Ok(id)
    }

    /// Get a resource by ID
    pub fn get_resource(&self, id: &Uuid) -> Option<&Resource> {
        self.resources.get(id)
    }

    /// Update a resource
    pub fn update_resource(&mut self, id: &Uuid, resource: Resource) -> Result<(), EngineError> {
        if self.resources.contains_key(id) {
            self.resources.insert(*id, resource);
            Ok(())
        } else {
            Err(EngineError::ResourceNotFound(format!(
                "Resource with ID {} not found",
                id
            )))
        }
    }

    /// Delete a resource
    pub fn delete_resource(&mut self, id: &Uuid) -> Result<(), EngineError> {
        if self.resources.remove(id).is_some() {
            Ok(())
        } else {
            Err(EngineError::ResourceNotFound(format!(
                "Resource with ID {} not found",
                id
            )))
        }
    }

    /// List all resources
    pub fn list_resources(&self) -> Vec<&Resource> {
        self.resources.values().collect()
    }
}