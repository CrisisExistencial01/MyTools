use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Docker daemon unreachable: {0}")]
    DockerConnection(String),

    #[error("TUI error: {0}")]
    Tui(String),
}

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            AppError::Config(e) => format!("Config error: {e}"),
            AppError::Io(e) => format!("IO error: {e}"),
            AppError::DockerConnection(msg) => {
                format!("Cannot connect to Docker daemon: {msg}. Is it running?")
            }
            AppError::Tui(msg) => format!("TUI error: {msg}"),
        }
    }
}
