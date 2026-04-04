mod detectors;

use crate::domain::InitSystem;
use detectors::DETECTORS;

pub fn detect_init_system() -> InitSystem {
    DETECTORS
        .iter()
        .find(|d| d.markers_exist())
        .map(|d| d.system)
        .unwrap_or(InitSystem::None)
}
