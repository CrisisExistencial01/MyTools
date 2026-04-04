use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct LayoutRects {
    pub main: Rect,
    pub statusbar: Rect,
}

pub fn build_layout(area: Rect) -> LayoutRects {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    LayoutRects {
        main: chunks[0],
        statusbar: chunks[1],
    }
}
