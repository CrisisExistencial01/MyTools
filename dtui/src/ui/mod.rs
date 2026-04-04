use crate::app::AppState;
use ratatui::Frame;

mod layout;

pub fn render(frame: &mut Frame, state: &AppState) {
    let rects = layout::build_layout(frame.area());

    // Main area: placeholder
    use ratatui::widgets::{Block, Borders, Paragraph};
    let block = Block::default().title(" dtui ").borders(Borders::ALL);
    let text = format!(
        "dtui — Docker TUI\n\nInit system: {}\n\nPress 'q' to quit",
        state.init_system
    );
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, rects.main);

    // Status bar
    let statusbar = ratatui::widgets::Paragraph::new(" Ready ")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray));
    frame.render_widget(statusbar, rects.statusbar);
}
