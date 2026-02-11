//! gRPC service definitions for the infrastructure engine

use tonic::{Request, Response, Status};
use crate::models::*;
use crate::resources::*;

/// gRPC service for resource management
#[tonic::async_trait]
pub trait ResourceManagerService {
    /// Create a new resource
    async fn create_resource(
        &self,
        request: Request<Resource>,
    ) -> Result<Response<Resource>, Status>;

    /// Get a resource by ID
    async fn get_resource(
        &self,
        request: Request<GetResourceRequest>,
    ) -> Result<Response<Resource>, Status>;

    /// Update a resource
    async fn update_resource(
        &self,
        request: Request<Resource>,
    ) -> Result<Response<Resource>, Status>;

    /// Delete a resource
    async fn delete_resource(
        &self,
        request: Request<DeleteResourceRequest>,
    ) -> Result<Response<DeleteResourceResponse>, Status>;

    /// List all resources
    async fn list_resources(
        &self,
        request: Request<ListResourcesRequest>,
    ) -> Result<Response<ListResourcesResponse>, Status>;
}

/// Request for getting a resource
#[derive(Debug, Clone, tonic::ProtoBuf)]
pub struct GetResourceRequest {
    /// Resource ID
    pub id: String,
}

/// Request for deleting a resource
#[derive(Debug, Clone, tonic::ProtoBuf)]
pub struct DeleteResourceRequest {
    /// Resource ID
    pub id: String,
}

/// Response for deleting a resource
#[derive(Debug, Clone, tonic::ProtoBuf)]
pub struct DeleteResourceResponse {
    /// Success status
    pub success: bool,
}

/// Request for listing resources
#[derive(Debug, Clone, tonic::ProtoBuf)]
pub struct ListResourcesRequest {
    /// Page size
    pub page_size: i32,
    /// Page token
    pub page_token: String,
}

/// Response for listing resources
#[derive(Debug, Clone, tonic::ProtoBuf)]
pub struct ListResourcesResponse {
    /// Resources
    pub resources: Vec<Resource>,
    /// Next page token
    pub next_page_token: String,
}