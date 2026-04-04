use ratatui::layout::{Constraint, Direction, Layout, Rect};

const TABLE_HEADER_ROWS: u16 = 1;
const TABLE_BORDER_ROWS: u16 = 2;
const TABLE_CHROME_ROWS: u16 = TABLE_HEADER_ROWS + TABLE_BORDER_ROWS;

pub struct LayoutRects {
    pub list: Rect,
    pub details: Option<Rect>,
    pub statusbar: Rect,
    pub container_visible_rows: usize,
}

pub fn build_layout(area: Rect, show_details: bool) -> LayoutRects {
    if !show_details {
        let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        return LayoutRects {
            list: chunks[0],
            details: None,
            statusbar: chunks[1],
            container_visible_rows: chunks[0].height.saturating_sub(TABLE_CHROME_ROWS) as usize,
        };
    }

    let vertical = ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let horizontal = ratatui::layout::Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical[0]);

    let list_visible_rows = horizontal[0].height.saturating_sub(TABLE_CHROME_ROWS) as usize;

    LayoutRects {
        list: horizontal[0],
        details: Some(horizontal[1]),
        statusbar: vertical[1],
        container_visible_rows: list_visible_rows,
    }
}
