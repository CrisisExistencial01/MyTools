#![allow(dead_code, unused_imports)]

mod domain;
mod config;
mod init;
mod ui;
mod app;
mod tui;
mod docker;

use app::App;
use color_eyre::eyre::Result;
use config::Config;
use docker::{BollardClient, DockerService};
use domain::AppError;
use init::detect_init_system;
use std::time::Duration;
use tui::Tui;

async fn run(
    tui: &mut Tui,
    app: &mut App,
    config: &Config,
    docker: &dyn DockerService,
) -> Result<(), AppError> {
    use domain::ControlFlow;

    app.domain.containers = docker.list_containers(true).await?;
    if !app.domain.containers.is_empty() {
        app.view.selected_container = Some(0);
    }

    loop {
        if let Some(event) = tui.poll_event(Duration::from_millis(100)) {
            match app.handle_event(event, config) {
                ControlFlow::Quit => break,
                ControlFlow::Continue => {
                    tui.render(|frame| {
                        crate::ui::render(frame, &app.domain, &mut app.view);
                    })?;
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config = Config::load()?;
    let init_system = detect_init_system();
    let docker = BollardClient::new()?;

    let mut tui = Tui::new()?;
    tui.enter()?;

    let (w, h) = tui.size()?;
    let mut app = App::new(init_system, w, h);

    let result = run(&mut tui, &mut app, &config, &docker).await;

    tui.exit();
    result?;

    Ok(())
}
