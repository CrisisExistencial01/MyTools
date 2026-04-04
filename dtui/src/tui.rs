use crate::domain::{AppError, AppEvent};
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::Duration;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self, AppError> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)
            .map_err(|e| AppError::Tui(format!("Failed to create terminal: {e}")))?;
        Ok(Tui { terminal })
    }

    pub fn enter(&mut self) -> Result<(), AppError> {
        crossterm::terminal::enable_raw_mode()
            .map_err(|e| AppError::Tui(format!("Failed to enable raw mode: {e}")))?;
        crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)
            .map_err(|e| AppError::Tui(format!("Failed to enter alternate screen: {e}")))?;
        Ok(())
    }

    pub fn exit(&mut self) {
        let _ = crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();
    }

    pub fn poll_event(&self, timeout: Duration) -> Option<AppEvent> {
        if event::poll(timeout).ok()? {
            match event::read().ok()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => Some(AppEvent::KeyInput(key)),
                Event::Resize(w, h) => Some(AppEvent::Resize(w, h)),
                _ => None,
            }
        } else {
            Some(AppEvent::Tick)
        }
    }

    pub fn size(&self) -> Result<(u16, u16), std::io::Error> {
        crossterm::terminal::size()
    }

    pub fn render<F>(&mut self, f: F) -> Result<(), AppError>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal
            .draw(f)
            .map_err(|e| AppError::Tui(format!("Failed to draw: {e}")))?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit();
    }
}
