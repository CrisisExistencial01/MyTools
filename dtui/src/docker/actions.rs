use crate::domain::AppError;
use crate::docker::service::DockerService;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerAction {
    Start,
    Stop,
    Restart,
    Pause,
    Unpause,
    Kill,
}

impl ContainerAction {
    pub fn display_name(&self) -> &'static str {
        match self {
            ContainerAction::Start => "Start",
            ContainerAction::Stop => "Stop",
            ContainerAction::Restart => "Restart",
            ContainerAction::Pause => "Pause",
            ContainerAction::Unpause => "Unpause",
            ContainerAction::Kill => "Kill",
        }
    }

    pub fn verb(&self) -> &'static str {
        match self {
            ContainerAction::Start => "start",
            ContainerAction::Stop => "stop",
            ContainerAction::Restart => "restart",
            ContainerAction::Pause => "pause",
            ContainerAction::Unpause => "unpause",
            ContainerAction::Kill => "kill",
        }
    }

    pub async fn execute(&self, docker: &dyn DockerService, id: &str) -> Result<(), AppError> {
        match self {
            ContainerAction::Start => docker.start_container(id).await,
            ContainerAction::Stop => docker.stop_container(id).await,
            ContainerAction::Restart => docker.restart_container(id).await,
            ContainerAction::Pause => docker.pause_container(id).await,
            ContainerAction::Unpause => docker.unpause_container(id).await,
            ContainerAction::Kill => docker.kill_container(id).await,
        }
    }
}
