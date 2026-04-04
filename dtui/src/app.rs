use crate::config::Config;
use crate::domain::{AppAction, AppEvent, Container, ControlFlow, InitSystem, Panel};

pub struct DomainState {
    pub containers: Vec<Container>,
    pub init_system: InitSystem,
}

impl DomainState {
    pub fn new(init_system: InitSystem) -> Self {
        DomainState {
            containers: Vec::new(),
            init_system,
        }
    }

    pub fn container_count(&self) -> usize {
        self.containers.len()
    }
}

pub struct Viewport {
    pub width: u16,
    pub height: u16,
    pub visible_rows: usize,
}

impl Viewport {
    pub fn new(width: u16, height: u16) -> Self {
        Viewport {
            width,
            height,
            visible_rows: height.saturating_sub(2) as usize,
        }
    }
}

pub struct ViewState {
    pub active_panel: Panel,
    pub selected_container: Option<usize>,
    pub scroll_offset: usize,
    pub viewport: Viewport,
}

impl ViewState {
    pub fn new(width: u16, height: u16) -> Self {
        ViewState {
            active_panel: Panel::default(),
            selected_container: None,
            scroll_offset: 0,
            viewport: Viewport::new(width, height),
        }
    }

    pub fn update_viewport(&mut self, width: u16, height: u16) {
        self.viewport.width = width;
        self.viewport.height = height;
    }

    pub fn apply_action(&mut self, action: AppAction, container_count: usize) {
        match action {
            AppAction::SelectUp => {
                let (sel, scroll) = select_up(
                    self.selected_container,
                    self.scroll_offset,
                    container_count,
                    self.viewport.visible_rows,
                );
                self.selected_container = sel;
                self.scroll_offset = scroll;
            }
            AppAction::SelectDown => {
                let (sel, scroll) = select_down(
                    self.selected_container,
                    self.scroll_offset,
                    container_count,
                    self.viewport.visible_rows,
                );
                self.selected_container = sel;
                self.scroll_offset = scroll;
            }
            AppAction::NextPanel | AppAction::PrevPanel => {
                self.active_panel = toggle_panel(self.active_panel);
            }
            AppAction::Quit => {}
        }
    }
}

pub struct App {
    pub domain: DomainState,
    pub view: ViewState,
}

impl App {
    pub fn new(init_system: InitSystem, width: u16, height: u16) -> Self {
        App {
            domain: DomainState::new(init_system),
            view: ViewState::new(width, height),
        }
    }

    pub fn handle_event(&mut self, event: AppEvent, config: &Config) -> ControlFlow {
        match event {
            AppEvent::KeyInput(key) => {
                if let Some(action) = config.keybindings.lookup(&key) {
                    if action == AppAction::Quit {
                        return ControlFlow::Quit;
                    }
                    self.view
                        .apply_action(action, self.domain.container_count());
                }
            }
            AppEvent::Resize(w, h) => {
                self.view.update_viewport(w, h);
            }
            AppEvent::Quit => return ControlFlow::Quit,
            AppEvent::Tick => {}
        }
        ControlFlow::Continue
    }
}

fn select_up(
    selected: Option<usize>,
    scroll_offset: usize,
    total: usize,
    visible_rows: usize,
) -> (Option<usize>, usize) {
    if total == 0 {
        return (selected, scroll_offset);
    }
    let idx = selected.unwrap_or(0);
    let new_idx = (idx + total - 1) % total;
    let new_scroll = if new_idx == total - 1 && idx == 0 {
        total.saturating_sub(visible_rows)
    } else if scroll_offset > new_idx {
        new_idx
    } else {
        scroll_offset
    };
    (Some(new_idx), new_scroll)
}

fn select_down(
    selected: Option<usize>,
    scroll_offset: usize,
    total: usize,
    visible_rows: usize,
) -> (Option<usize>, usize) {
    if total == 0 {
        return (selected, scroll_offset);
    }
    let idx = selected.unwrap_or(0);
    let new_idx = (idx + 1) % total;
    let new_scroll = if new_idx == 0 && idx == total - 1 {
        0
    } else if new_idx >= scroll_offset + visible_rows {
        new_idx.saturating_sub(visible_rows) + 1
    } else {
        scroll_offset
    };
    (Some(new_idx), new_scroll)
}

fn toggle_panel(current: Panel) -> Panel {
    match current {
        Panel::Containers => Panel::Volumes,
        Panel::Volumes => Panel::Containers,
    }
}
