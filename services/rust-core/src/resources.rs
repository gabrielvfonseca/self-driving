//! Resource management functionality.

use crate::models::*;
use std::collections::HashMap;

/// Resource manager for infrastructure provisioning
pub struct ResourceManager {
    /// Map of all managed resources
    pub resources: HashMap<String, Resource>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Create a new resource
    pub fn create_resource(&mut self, request: ResourceRequest) -> ResourceResponse {
        let resource_id = format!("res-{}", uuid::Uuid::new_v4());
        let now = chrono::Utc::now().timestamp();

        let resource = Resource {
            id: resource_id.clone(),
            name: request.name,
            resource_type: request.resource_type,
            status: ResourceStatus::Pending,
            provider: request.provider.unwrap_or_else(|| "default".to_string()),
            metadata: ResourceMetadata {
                specs: request.specs,
                labels: request.labels,
                annotations: request.annotations,
            },
            created_at: now,
            updated_at: now,
        };

        self.resources.insert(resource_id, resource.clone());

        ResourceResponse {
            resource,
            status: ResourceStatus::Pending,
            error: None,
        }
    }

    /// Get a resource by ID
    pub fn get_resource(&self, id: &str) -> Option<&Resource> {
        self.resources.get(id)
    }

    /// Update a resource
    pub fn update_resource(&mut self, id: &str, update: ResourceUpdate) -> Result<Resource, String> {
        if let Some(resource) = self.resources.get_mut(id) {
            // Update the resource fields
            resource.updated_at = chrono::Utc::now().timestamp();
            resource.status = update.status.unwrap_or(resource.status);
            
            // Update metadata if provided
            if let Some(specs) = update.specs {
                resource.metadata.specs = specs;
            }
            
            Ok(resource.clone())
        } else {
            Err(format!("Resource with ID {} not found", id))
        }
    }

    /// Delete a resource
    pub fn delete_resource(&mut self, id: &str) -> Result<Resource, String> {
        self.resources.remove(id)
            .ok_or_else(|| format!("Resource with ID {} not found", id))
    }

    /// List all resources
    pub fn list_resources(&self) -> Vec<&Resource> {
        self.resources.values().collect()
    }
}

/// Resource update structure
#[derive(Debug, Clone)]
pub struct ResourceUpdate {
    /// New status
    pub status: Option<ResourceStatus>,
    /// New specs
    pub specs: Option<ResourceSpecs>,
}
