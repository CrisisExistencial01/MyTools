#![allow(dead_code, unused_imports)]

mod domain;
mod config;
mod init;
mod ui;
mod app;
mod tui;
mod docker;
mod shell;

use app::App;
use color_eyre::eyre::Result;
use config::Config;
use docker::{BollardClient, DockerService};
use domain::{AppAction, AppError, AppEvent, ControlFlow};
use init::detect_init_system;
use shell::AppContext;
use std::time::Duration;
use tui::Tui;

async fn run(ctx: &mut AppContext<'_>) -> Result<(), AppError> {
    ctx.app.domain.containers = ctx.docker.list_containers(true).await?;
    if !ctx.app.domain.containers.is_empty() { ctx.app.view.selected_container = Some(0); }

    loop {
        while let Ok(event) = ctx.event_rx.try_recv() { ctx.app.handle_event(event); }

        if ctx.app.view.needs_refresh {
            ctx.app.view.needs_refresh = false;
            if let Ok(containers) = ctx.docker.list_containers(true).await {
                ctx.app.domain.containers = containers;
            }
        }

        if let Some(event) = ctx.tui.poll_event(Duration::from_millis(100)) {
            let control = match &event {
                AppEvent::KeyInput(key) => {
                    use crossterm::event::KeyCode;
                    if ctx.app.view.show_help {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                            ctx.app.handle_action(AppAction::ToggleHelp);
                        }
                        ControlFlow::Continue
                    } else if ctx.app.view.palette.open {
                        match key.code {
                            KeyCode::Esc => ctx.app.handle_action(AppAction::ClosePalette),
                            KeyCode::Enter => {
                                let all = crate::ui::palette::palette_commands();
                                let filtered: Vec<_> = all.iter()
                                    .filter(|e| e.key.contains(&ctx.app.view.palette.filter) || e.desc.to_lowercase().contains(&ctx.app.view.palette.filter))
                                    .collect();
                                if let Some(entry) = filtered.get(ctx.app.view.palette.cursor) {
                                    let op = entry.action;
                                    if let Some(cid) = ctx.app.selected_container_id() {
                                        let tx = ctx.event_tx.clone();
                                        let docker_clone = ctx.docker.clone();
                                        let cid_clone = cid.clone();
                                        tokio::spawn(async move {
                                            let res = op.execute(&docker_clone, &cid_clone).await;
                                            let _ = tx.send(AppEvent::OperationComplete { action: op, container_id: cid_clone, result: res.map_err(|e| e.to_string()) });
                                        });
                                    }
                                    ctx.app.view.palette.close();
                                }
                                ControlFlow::Continue
                            }
                            KeyCode::Char(c) => { ctx.app.handle_action(AppAction::PaletteChar(c)); ControlFlow::Continue }
                            KeyCode::Backspace => { ctx.app.handle_action(AppAction::PaletteBackspace); ControlFlow::Continue }
                            _ => ControlFlow::Continue,
                        }
                    } else if let Some(action) = ctx.config.keybindings.lookup(key) {
                        ctx.app.handle_action(action)
                    } else { ControlFlow::Continue }
                }
                _ => { ctx.app.handle_event(event); ControlFlow::Continue }
            };

            if let Some(cid) = ctx.app.selected_container_id() {
                if ctx.app.view.needs_details_fetch(Some(&cid)) {
                    let tx = ctx.event_tx.clone();
                    let docker_clone = ctx.docker.clone();
                    let cid_clone = cid.clone();
                    tokio::spawn(async move {
                        let res = docker_clone.inspect_container(&cid_clone).await;
                        let _ = tx.send(AppEvent::DetailsFetched { container_id: cid_clone, result: res.map_err(|e| e.to_string()) });
                    });
                    ctx.app.view.mark_details_fetched(&cid);
                }
            }

            match control {
                ControlFlow::Quit => break,
                ControlFlow::Continue => {
                    ctx.tui.render(|frame| { crate::ui::render(frame, &ctx.app.domain, &mut ctx.app.view, ctx.config); })?;
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
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let mut ctx = AppContext::new(&mut tui, &mut app, &config, docker, event_tx, &event_rx);
    let result = run(&mut ctx).await;
    ctx.tui.exit();
    result?;
    Ok(())
}
