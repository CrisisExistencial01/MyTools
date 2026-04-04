use async_trait::async_trait;
use bollard::Docker;
use bollard::query_parameters::{
    InspectContainerOptions, KillContainerOptions, ListContainersOptions,
    RestartContainerOptions, StopContainerOptions, StartContainerOptions,
};
use chrono::{DateTime, Utc};
use std::time::Duration;
use crate::domain::{AppError, Container, ContainerState, PortMapping};
use crate::docker::service::{ContainerDetails, DockerService};

pub struct BollardClient {
    docker: Docker,
}

impl Clone for BollardClient {
    fn clone(&self) -> Self {
        BollardClient {
            docker: self.docker.clone(),
        }
    }
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

    async fn inspect_container(&self, id: &str) -> Result<ContainerDetails, AppError> {
        let info = self.docker.inspect_container(id, None::<InspectContainerOptions>).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;

        let state = info.state.as_ref();
        let container_state = state.map(|s| {
            if s.running.unwrap_or(false) {
                if s.paused.unwrap_or(false) {
                    ContainerState::Paused
                } else {
                    ContainerState::Running
                }
            } else if s.restarting.unwrap_or(false) {
                ContainerState::Restarting
            } else if s.dead.unwrap_or(false) {
                ContainerState::Dead
            } else if s.oom_killed.unwrap_or(false) {
                ContainerState::Exited
            } else {
                ContainerState::Exited
            }
        }).unwrap_or(ContainerState::Created);

        // Compute uptime
        let uptime = state.and_then(|s| {
            if s.running.unwrap_or(false) {
                s.started_at.as_ref().and_then(|started| {
                    DateTime::parse_from_rfc3339(started).ok()
                        .map(|dt| Utc::now().signed_duration_since(dt).to_std().ok())
                        .flatten()
                })
            } else {
                None
            }
        });

        let created_at = info.created.as_ref().and_then(|c| {
            DateTime::parse_from_rfc3339(c).ok().map(|dt| dt.with_timezone(&Utc))
        });

        let started_at = state.and_then(|s| {
            s.started_at.as_ref().and_then(|c| {
                DateTime::parse_from_rfc3339(c).ok().map(|dt| dt.with_timezone(&Utc))
            })
        });

        let ports: Vec<PortMapping> = info.network_settings.as_ref()
            .and_then(|ns| ns.ports.as_ref())
            .map(|ports| {
                ports.iter().flat_map(|(container_port_proto, host_bindings)| {
                    let parts: Vec<&str> = container_port_proto.split('/').collect();
                    let container_port: u16 = parts.first().and_then(|p| p.parse().ok()).unwrap_or(0);
                    let protocol = parts.get(1).map(|s| s.to_string()).unwrap_or_else(|| "tcp".to_string());

                    host_bindings.iter().flatten().map(|binding| {
                        PortMapping {
                            host_ip: binding.host_ip.clone().unwrap_or_default(),
                            host_port: binding.host_port.as_ref().and_then(|p| p.parse().ok()).unwrap_or(0),
                            container_port,
                            protocol: protocol.clone(),
                        }
                    }).collect::<Vec<_>>()
                }).collect()
            })
            .unwrap_or_default();

        let labels = info.config.as_ref()
            .and_then(|c| c.labels.as_ref())
            .map(|l| l.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();

        Ok(ContainerDetails {
            id: info.id.unwrap_or_default().chars().take(12).collect(),
            name: info.name.unwrap_or_default().trim_start_matches('/').to_string(),
            image: info.config.as_ref().and_then(|c| c.image.clone()).unwrap_or_default(),
            state: container_state,
            status: state.and_then(|s| s.status.as_ref())
                .map(|s| format!("{s:?}")).unwrap_or_default(),
            uptime,
            ports,
            created_at,
            started_at,
            command: info.config.as_ref().and_then(|c| c.cmd.as_ref())
                .map(|cmd| cmd.join(" ")).unwrap_or_default(),
            labels,
        })
    }

    async fn start_container(&self, id: &str) -> Result<(), AppError> {
        self.docker.start_container(id, None::<StartContainerOptions>).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(())
    }

    async fn stop_container(&self, id: &str) -> Result<(), AppError> {
        self.docker.stop_container(id, None::<StopContainerOptions>).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(())
    }

    async fn restart_container(&self, id: &str) -> Result<(), AppError> {
        self.docker.restart_container(id, None::<RestartContainerOptions>).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(())
    }

    async fn pause_container(&self, id: &str) -> Result<(), AppError> {
        self.docker.pause_container(id).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(())
    }

    async fn unpause_container(&self, id: &str) -> Result<(), AppError> {
        self.docker.unpause_container(id).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(())
    }

    async fn kill_container(&self, id: &str) -> Result<(), AppError> {
        self.docker.kill_container(id, None::<KillContainerOptions>).await
            .map_err(|e| AppError::DockerConnection(e.to_string()))?;
        Ok(())
    }
}
