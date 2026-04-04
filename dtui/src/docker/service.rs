use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::time::Duration;
use crate::domain::{AppError, Container, ContainerState, PortMapping};

#[derive(Debug, Clone)]
pub struct ContainerDetails {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
    pub uptime: Option<Duration>,
    pub ports: Vec<PortMapping>,
    pub created_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub command: String,
    pub labels: Vec<(String, String)>,
}

#[async_trait]
pub trait DockerService: Send + Sync {
    async fn list_containers(&self, all: bool) -> Result<Vec<Container>, AppError>;
    async fn inspect_container(&self, id: &str) -> Result<ContainerDetails, AppError>;
    async fn start_container(&self, id: &str) -> Result<(), AppError>;
    async fn stop_container(&self, id: &str) -> Result<(), AppError>;
    async fn restart_container(&self, id: &str) -> Result<(), AppError>;
    async fn pause_container(&self, id: &str) -> Result<(), AppError>;
    async fn unpause_container(&self, id: &str) -> Result<(), AppError>;
    async fn kill_container(&self, id: &str) -> Result<(), AppError>;
}
