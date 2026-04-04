use crate::config::Config;
use crate::domain::{AppError, AppEvent, InitSystem, Panel};
use crate::tui::Tui;
use std::time::Duration;

pub struct AppState {
    pub init_system: InitSystem,
    pub active_panel: Panel,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(init_system: InitSystem) -> Self {
        AppState {
            init_system,
            active_panel: Panel::default(),
            should_quit: false,
        }
    }

    pub fn handle_event(&mut self, event: AppEvent, config: &Config) {
        match event {
            AppEvent::KeyInput(key) => {
                if let Some(action) = config.keybindings.lookup(&key) {
                    match action {
                        "Quit" => self.should_quit = true,
                        "NextPanel" => {
                            self.active_panel = match self.active_panel {
                                Panel::Containers => Panel::Volumes,
                                Panel::Volumes => Panel::Containers,
                            }
                        }
                        "PrevPanel" => {
                            self.active_panel = match self.active_panel {
                                Panel::Containers => Panel::Volumes,
                                Panel::Volumes => Panel::Containers,
                            }
                        }
                        _ => {}
                    }
                }
            }
            AppEvent::Resize(_, _) => {}
            AppEvent::Tick => {}
            AppEvent::Quit => self.should_quit = true,
        }
    }
}

pub fn run(tui: &mut Tui, state: &mut AppState, config: &Config) -> Result<(), AppError> {
    while !state.should_quit {
        tui.render(|frame| {
            crate::ui::render(frame, state);
        })?;

        if let Some(event) = tui.poll_event(Duration::from_millis(100)) {
            state.handle_event(event, config);
        }
    }

    Ok(())
}
