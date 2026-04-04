use crate::app::{DomainState, ViewState};
use ratatui::Frame;

mod containers;
mod layout;

pub fn render(frame: &mut Frame, domain: &DomainState, view: &mut ViewState) {
    let rects = layout::build_layout(frame.area());
    view.viewport.visible_rows = rects.container_visible_rows;

    match view.active_panel {
        crate::domain::Panel::Containers => {
            containers::render_containers(frame, domain, view, rects.main);
        }
        crate::domain::Panel::Volumes => {
            use ratatui::widgets::{Block, Borders, Paragraph};
            let block = Block::default().title(" Volumes ").borders(Borders::ALL);
            frame.render_widget(
                Paragraph::new("Volumes panel (coming soon)").block(block),
                rects.main,
            );
        }
    }

    let statusbar = ratatui::widgets::Paragraph::new(" Ready ")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray));
    frame.render_widget(statusbar, rects.statusbar);
}
