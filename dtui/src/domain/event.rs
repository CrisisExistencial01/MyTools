use crossterm::event::KeyEvent;

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
