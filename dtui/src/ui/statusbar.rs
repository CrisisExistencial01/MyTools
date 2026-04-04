use crate::app::{StatusMessage, StatusSeverity, ViewState};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_statusbar(frame: &mut Frame, view: &ViewState, area: Rect) {
    let text = if view.palette.open {
        " [/] type action... (Esc to cancel) ".to_string()
    } else if view.show_help {
        " [Help] press ? or Esc to close ".to_string()
    } else if !view.filter.text.is_empty() {
        format!(" 🔍 {} | f edit | Esc clear ", view.filter.text)
    } else {
        let base = match &view.status_message {
            Some(msg) => {
                let prefix = match msg.severity {
                    StatusSeverity::Success => "✓",
                    StatusSeverity::Error => "✗",
                    StatusSeverity::Info => "ℹ",
                };
                format!(" {prefix} {} ", msg.text)
            }
            None => " Ready ".to_string(),
        };
        format!("{}| / actions | ? help", base)
    };

    frame.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::DarkGray)),
        area,
    );
}
