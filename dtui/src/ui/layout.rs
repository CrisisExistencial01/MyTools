use ratatui::layout::{Constraint, Direction, Layout, Rect};

// Table chrome: 1 header row + 2 border rows (top + bottom)
const TABLE_HEADER_ROWS: u16 = 1;
const TABLE_BORDER_ROWS: u16 = 2;
const TABLE_CHROME_ROWS: u16 = TABLE_HEADER_ROWS + TABLE_BORDER_ROWS;

pub struct LayoutRects {
    pub main: Rect,
    pub statusbar: Rect,
    pub container_visible_rows: usize,
}

pub fn build_layout(area: Rect) -> LayoutRects {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let container_visible_rows = chunks[0].height.saturating_sub(TABLE_CHROME_ROWS) as usize;

    LayoutRects {
        main: chunks[0],
        statusbar: chunks[1],
        container_visible_rows,
    }
}
