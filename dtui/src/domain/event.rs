use crossterm::event::KeyEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppAction {
    Quit,
    SelectUp,
    SelectDown,
    NextPanel,
    PrevPanel,
}

pub enum AppEvent {
    KeyInput(KeyEvent),
    Tick,
    Resize(u16, u16),
    Quit,
}

pub type AppEventSender = std::sync::mpsc::Sender<AppEvent>;
pub type AppEventReceiver = std::sync::mpsc::Receiver<AppEvent>;

pub fn create_event_channel() -> (AppEventSender, AppEventReceiver) {
    std::sync::mpsc::channel()
}
