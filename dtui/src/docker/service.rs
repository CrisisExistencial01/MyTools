use async_trait::async_trait;
use crate::domain::{AppError, Container};

#[async_trait]
pub trait DockerService: Send + Sync {
    async fn list_containers(&self, all: bool) -> Result<Vec<Container>, AppError>;
    async fn inspect_container(&self, id: &str) -> Result<Container, AppError>;
}
