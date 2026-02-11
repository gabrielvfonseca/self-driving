//! Data models for infrastructure resources

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base resource structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Unique identifier for the resource
    pub id: Uuid,
    /// Name of the resource
    pub name: String,
    /// Type of the resource
    pub resource_type: ResourceType,
    /// Provider identifier
    pub provider: Provider,
    /// Resource status
    pub status: ResourceStatus,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Resource specifications
    pub spec: ResourceSpec,
    /// Resource status details
    pub status_details: Option<String>,
}

/// Resource type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Cloud provider enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Provider {
    /// Amazon Web Services
    Aws,
    /// Google Cloud Platform
    Gcp,
    /// Microsoft Azure
    Azure,
    /// On-premise
    OnPremise,
}

/// Resource status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceStatus {
    /// Resource is pending creation
    Pending,
    /// Resource is active
    Active,
    /// Resource is updating
    Updating,
    /// Resource is deleting
    Deleting,
    /// Resource has failed
    Failed,
    /// Resource is suspended
    Suspended,
}

/// Resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// Common fields for all resources
    pub common: CommonSpec,
    /// Provider-specific fields
    pub provider_spec: ProviderSpec,
}

/// Common resource specification fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonSpec {
    /// Resource description
    pub description: Option<String>,
    /// Resource tags
    pub tags: std::collections::HashMap<String, String>,
    /// Resource labels
    pub labels: std::collections::HashMap<String, String>,
    /// Resource region
    pub region: String,
    /// Resource zone (for zonal resources)
    pub zone: Option<String>,
    /// Resource availability zone
    pub availability_zone: Option<String>,
    /// Resource cost estimate
    pub cost_estimate: Option<f64>,
    /// Resource dependencies
    pub dependencies: Vec<Uuid>,
}

/// Provider-specific resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSpec {
    /// AWS-specific fields
    pub aws: Option<AwsSpec>,
    /// GCP-specific fields
    pub gcp: Option<GcpSpec>,
    /// Azure-specific fields
    pub azure: Option<AzureSpec>,
}

/// AWS-specific resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsSpec {
    /// Instance type
    pub instance_type: Option<String>,
    /// AMI ID
    pub ami_id: Option<String>,
    /// Security groups
    pub security_groups: Vec<String>,
    /// Subnet ID
    pub subnet_id: Option<String>,
    /// VPC ID
    pub vpc_id: Option<String>,
    /// EBS volume specifications
    pub ebs_volumes: Vec<EbsVolume>,
    /// IAM role
    pub iam_role: Option<String>,
    /// Auto scaling group
    pub auto_scaling_group: Option<String>,
}

/// EBS volume specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbsVolume {
    /// Volume size in GB
    pub size_gb: u32,
    /// Volume type
    pub volume_type: String,
    /// IOPS (for io1 volumes)
    pub iops: Option<u32>,
    /// Encrypted
    pub encrypted: bool,
}

/// GCP-specific resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpSpec {
    /// Machine type
    pub machine_type: Option<String>,
    /// Image name
    pub image: Option<String>,
    /// Network
    pub network: Option<String>,
    /// Subnetwork
    pub subnetwork: Option<String>,
    /// Disks
    pub disks: Vec<GcpDisk>,
    /// Service account
    pub service_account: Option<String>,
    /// Auto scaling
    pub auto_scaling: Option<bool>,
}

/// GCP disk specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpDisk {
    /// Disk size in GB
    pub size_gb: u32,
    /// Disk type
    pub disk_type: String,
    /// Interface
    pub interface: String,
    /// Boot disk
    pub boot: bool,
}

/// Azure-specific resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureSpec {
    /// VM size
    pub vm_size: Option<String>,
    /// Image reference
    pub image_reference: Option<ImageReference>,
    /// Network interface
    pub network_interface: Option<String>,
    /// Storage account
    pub storage_account: Option<String>,
    /// Availability set
    pub availability_set: Option<String>,
    /// Disks
    pub disks: Vec<AzureDisk>,
}

/// Image reference for Azure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageReference {
    /// Publisher
    pub publisher: String,
    /// Offer
    pub offer: String,
    /// SKU
    pub sku: String,
    /// Version
    pub version: String,
}

/// Azure disk specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureDisk {
    /// Disk size in GB
    pub size_gb: u32,
    /// Disk type
    pub disk_type: String,
    /// Managed disk
    pub managed: bool,
}