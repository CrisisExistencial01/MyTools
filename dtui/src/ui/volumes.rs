use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_volumes(frame: &mut Frame, area: Rect) {
    let block = Block::default().title(" Volumes ").borders(Borders::ALL);
    frame.render_widget(
        Paragraph::new("Volumes panel (coming soon)").block(block),
        area,
    );
}
