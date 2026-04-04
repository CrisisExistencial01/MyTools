use async_trait::async_trait;
use bollard::Docker;
use bollard::query_parameters::ListContainersOptions;
use crate::domain::{AppError, Container, ContainerState, PortMapping};
use crate::docker::service::DockerService;

pub struct BollardClient {
    docker: Docker,
}

impl BollardClient {
    pub fn new() -> Result<Self, AppError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(BollardClient { docker })
    }
}

#[async_trait]
impl DockerService for BollardClient {
    async fn list_containers(&self, all: bool) -> Result<Vec<Container>, AppError> {
        let options = Some(ListContainersOptions {
            all,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;

        let result: Vec<Container> = containers.into_iter().map(|c| {
            Container {
                id: c.id.unwrap_or_default().chars().take(12).collect(),
                name: c.names.unwrap_or_default().first()
                    .map(|n| n.trim_start_matches('/').to_string())
                    .unwrap_or_default(),
                image: c.image.unwrap_or_default(),
                state: c.state.map_or(ContainerState::default(), |s| match s {
                    bollard::models::ContainerSummaryStateEnum::RUNNING => ContainerState::Running,
                    bollard::models::ContainerSummaryStateEnum::PAUSED => ContainerState::Paused,
                    bollard::models::ContainerSummaryStateEnum::RESTARTING => ContainerState::Restarting,
                    bollard::models::ContainerSummaryStateEnum::EXITED => ContainerState::Exited,
                    bollard::models::ContainerSummaryStateEnum::DEAD => ContainerState::Dead,
                    bollard::models::ContainerSummaryStateEnum::REMOVING => ContainerState::Removing,
                    _ => ContainerState::Created,
                }),
                status: c.status.unwrap_or_default(),
                ports: c.ports.unwrap_or_default().into_iter().map(|p| PortMapping {
                    host_ip: p.ip.unwrap_or_default(),
                    host_port: p.public_port.unwrap_or_default() as u16,
                    container_port: p.private_port as u16,
                    protocol: format!("{:?}", p.typ),
                }).collect(),
            }
        }).collect();

        Ok(result)
    }

    async fn inspect_container(&self, _id: &str) -> Result<Container, AppError> {
        Err(AppError::DockerConnection("Not implemented yet".into()))
    }
}
