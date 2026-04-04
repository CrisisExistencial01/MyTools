use crate::config::Config;
use crate::docker::{ContainerAction, ContainerDetails};
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

pub struct StatusMessage {
    pub text: String,
    pub severity: StatusSeverity,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusSeverity {
    Info,
    Success,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedPane {
    #[default]
    List,
    Details,
}

pub struct CommandPalette {
    pub open: bool,
    pub filter: String,
    pub cursor: usize,
}
impl CommandPalette {
    pub fn new() -> Self {
        Self {
            open: false,
            filter: String::new(),
            cursor: 0,
        }
    }
    pub fn open(&mut self) {
        self.open = true;
        self.filter.clear();
        self.cursor = 0;
    }
    pub fn close(&mut self) {
        self.open = false;
        self.filter.clear();
        self.cursor = 0;
    }
    pub fn push_char(&mut self, c: char) {
        self.filter.push(c);
        self.cursor = 0;
    }
    pub fn backspace(&mut self) {
        self.filter.pop();
        self.cursor = 0;
    }
}

pub struct FilterState {
    pub open: bool,
    pub text: String,
}
impl FilterState {
    pub fn new() -> Self {
        Self {
            open: false,
            text: String::new(),
        }
    }
    pub fn open(&mut self) {
        self.open = true;
        // Keep text so user can continue editing
    }
    pub fn close(&mut self) {
        self.open = false;
        // Keep text so filter persists
    }
    pub fn clear(&mut self) {
        self.open = false;
        self.text.clear();
    }
    pub fn push_char(&mut self, c: char) {
        self.text.push(c);
    }
    pub fn backspace(&mut self) {
        self.text.pop();
    }
}

pub struct ViewState {
    pub active_panel: Panel,
    pub focused_pane: FocusedPane,
    pub selected_container: Option<usize>,
    pub scroll_offset: usize,
    pub viewport: Viewport,
    pub status_message: Option<StatusMessage>,
    pub selected_details: Option<ContainerDetails>,
    pub last_fetched_id: Option<String>,
    pub show_details: bool,
    pub show_help: bool,
    pub palette: CommandPalette,
    pub filter: FilterState,
    pub needs_refresh: bool,
}

impl ViewState {
    pub fn new(width: u16, height: u16) -> Self {
        ViewState {
            active_panel: Panel::default(),
            focused_pane: FocusedPane::default(),
            selected_container: None,
            scroll_offset: 0,
            viewport: Viewport::new(width, height),
            status_message: None,
            selected_details: None,
            last_fetched_id: None,
            show_details: false,
            show_help: false,
            palette: CommandPalette::new(),
            filter: FilterState::new(),
            needs_refresh: false,
        }
    }
    pub fn update_viewport(&mut self, w: u16, h: u16) {
        self.viewport.width = w;
        self.viewport.height = h;
    }
    pub fn set_status(&mut self, text: String, sev: StatusSeverity) {
        self.status_message = Some(StatusMessage {
            text,
            severity: sev,
        });
    }
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
    pub fn toggle_focus(&mut self) {
        self.focused_pane = match self.focused_pane {
            FocusedPane::List => FocusedPane::Details,
            _ => FocusedPane::List,
        };
    }
    pub fn needs_details_fetch(&self, id: Option<&str>) -> bool {
        match (id, &self.last_fetched_id) {
            (Some(i), Some(f)) => i != f,
            (Some(_), None) => true,
            _ => false,
        }
    }
    pub fn mark_details_fetched(&mut self, id: &str) {
        self.last_fetched_id = Some(id.to_string());
    }

    pub fn clamp_selection(&mut self, total: usize) {
        if let Some(sel) = self.selected_container {
            if total == 0 {
                self.selected_container = None;
            } else if sel >= total {
                self.selected_container = Some(total - 1);
            }
        }
    }

    pub fn filtered_indices(&self, containers: &[Container]) -> Vec<usize> {
        if self.filter.text.is_empty() {
            return (0..containers.len()).collect();
        }
        let q = self.filter.text.to_lowercase();
        containers
            .iter()
            .enumerate()
            .filter(|(_, c)| c.name.to_lowercase().contains(&q) || c.id.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn apply_action(&mut self, action: AppAction, count: usize) {
        match action {
            AppAction::SelectUp => {
                let (s, o) = select_up(
                    self.selected_container,
                    self.scroll_offset,
                    count,
                    self.viewport.visible_rows,
                );
                self.selected_container = s;
                self.scroll_offset = o;
            }
            AppAction::SelectDown => {
                let (s, o) = select_down(
                    self.selected_container,
                    self.scroll_offset,
                    count,
                    self.viewport.visible_rows,
                );
                self.selected_container = s;
                self.scroll_offset = o;
            }
            AppAction::NextPanel | AppAction::PrevPanel => {
                self.active_panel = toggle_panel(self.active_panel);
            }
            AppAction::ToggleDetails => {
                self.show_details = !self.show_details;
                self.focused_pane = if self.show_details {
                    FocusedPane::Details
                } else {
                    FocusedPane::List
                };
            }
            AppAction::ToggleHelp => {
                self.show_help = !self.show_help;
            }
            AppAction::OpenPalette => {
                self.palette.open();
            }
            AppAction::ClosePalette => {
                self.palette.close();
            }
            AppAction::PaletteChar(c) => {
                self.palette.push_char(c);
            }
            AppAction::PaletteBackspace => {
                self.palette.backspace();
            }
            AppAction::Container(_) | AppAction::Quit => {}
            AppAction::OpenFilter => self.filter.open(),
            AppAction::CloseFilter => self.filter.close(),
            AppAction::FilterChar(c) => self.filter.push_char(c),
            AppAction::FilterBackspace => self.filter.backspace(),
        }
    }

    pub fn apply_operation_result(
        &mut self,
        action: &ContainerAction,
        result: &Result<(), String>,
    ) {
        match result {
            Ok(()) => {
                self.set_status(
                    format!("{} container", action.display_name()),
                    StatusSeverity::Success,
                );
                self.needs_refresh = true;
            }
            Err(e) => self.set_status(
                format!("Failed to {}: {e}", action.verb()),
                StatusSeverity::Error,
            ),
        }
    }
}

pub struct App {
    pub domain: DomainState,
    pub view: ViewState,
}
impl App {
    pub fn new(init: InitSystem, w: u16, h: u16) -> Self {
        App {
            domain: DomainState::new(init),
            view: ViewState::new(w, h),
        }
    }
    pub fn handle_action(&mut self, action: AppAction) -> ControlFlow {
        if action == AppAction::Quit {
            return ControlFlow::Quit;
        }
        self.view
            .apply_action(action, self.domain.container_count());
        ControlFlow::Continue
    }
    pub fn handle_event(&mut self, event: AppEvent, filtered: &[usize]) {
        match event {
            AppEvent::Resize(w, h) => self.view.update_viewport(w, h),
            AppEvent::OperationComplete {
                ref action,
                ref result,
                ..
            } => self.view.apply_operation_result(action, result),
            AppEvent::DetailsFetched {
                container_id,
                result,
            } => {
                if let Some(current_id) = self.selected_container_id(filtered) {
                    if container_id == current_id {
                        match result {
                            Ok(d) => self.view.selected_details = Some(d),
                            Err(e) => self.view.set_status(
                                format!("Failed to fetch details: {e}"),
                                StatusSeverity::Error,
                            ),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    pub fn selected_container_id(&self, filtered: &[usize]) -> Option<String> {
        self.view
            .selected_container
            .and_then(|i| filtered.get(i))
            .and_then(|&actual| self.domain.containers.get(actual))
            .map(|c| c.id.clone())
    }
}

fn select_up(sel: Option<usize>, off: usize, tot: usize, vis: usize) -> (Option<usize>, usize) {
    if tot == 0 {
        return (sel, off);
    }
    let idx = sel.unwrap_or(0);
    let new = (idx + tot - 1) % tot;
    let scroll = if new == tot - 1 && idx == 0 {
        tot.saturating_sub(vis)
    } else if off > new {
        new
    } else {
        off
    };
    (Some(new), scroll)
}
fn select_down(sel: Option<usize>, off: usize, tot: usize, vis: usize) -> (Option<usize>, usize) {
    if tot == 0 {
        return (sel, off);
    }
    let idx = sel.unwrap_or(0);
    let new = (idx + 1) % tot;
    let scroll = if new == 0 && idx == tot - 1 {
        0
    } else if new >= off + vis {
        new.saturating_sub(vis) + 1
    } else {
        off
    };
    (Some(new), scroll)
}
fn toggle_panel(p: Panel) -> Panel {
    match p {
        Panel::Containers => Panel::Volumes,
        _ => Panel::Containers,
    }
}
