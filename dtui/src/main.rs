#![allow(dead_code)]

mod app;
mod config;
mod domain;
mod init;
mod tui;
mod ui;

use app::{run, AppState};
use color_eyre::eyre::Result;
use config::Config;
use init::detect_init_system;
use tui::Tui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let config = Config::load()?;
    let init_system = detect_init_system();
    let mut state = AppState::new(init_system);
    let mut tui = Tui::new()?;
    tui.enter()?;

    let result = run(&mut tui, &mut state, &config);

    tui.exit();
    result?;

    Ok(())
}
