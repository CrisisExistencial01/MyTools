use crate::domain::InitSystem;
use std::path::Path;

pub struct InitDetector {
    pub system: InitSystem,
    pub markers: &'static [&'static str],
}

impl InitDetector {
    pub fn markers_exist(&self) -> bool {
        self.markers.iter().all(|p| Path::new(p).exists())
    }
}

pub const DETECTORS: &[InitDetector] = &[
    InitDetector {
        system: InitSystem::Systemd,
        markers: &["/run/systemd/system"],
    },
    InitDetector {
        system: InitSystem::S6Rc,
        markers: &["/usr/bin/s6-svscan", "/etc/s6-rc/compiled"],
    },
    InitDetector {
        system: InitSystem::S6,
        markers: &["/usr/bin/s6-svscan"],
    },
    InitDetector {
        system: InitSystem::Dinit,
        markers: &["/usr/bin/dinitctl"],
    },
    InitDetector {
        system: InitSystem::Openrc,
        markers: &["/usr/bin/rc-status", "/run/openrc"],
    },
    InitDetector {
        system: InitSystem::Runit,
        markers: &["/usr/bin/sv", "/run/runit"],
    },
];
