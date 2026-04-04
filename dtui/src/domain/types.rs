#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContainerState {
    #[default]
    Created,
    Running,
    Paused,
    Restarting,
    Exited,
    Dead,
    Removing,
}

#[derive(Debug, Clone, Default)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
}

#[derive(Debug, Clone, Default)]
pub struct Volume {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub containers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InitSystem {
    Systemd,
    S6,
    S6Rc,
    Dinit,
    Openrc,
    Runit,
    #[default]
    None,
}

impl std::fmt::Display for InitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitSystem::Systemd => write!(f, "systemd"),
            InitSystem::S6 => write!(f, "s6"),
            InitSystem::S6Rc => write!(f, "s6-rc"),
            InitSystem::Dinit => write!(f, "dinit"),
            InitSystem::Openrc => write!(f, "openrc"),
            InitSystem::Runit => write!(f, "runit"),
            InitSystem::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Panel {
    #[default]
    Containers,
    Volumes,
}
