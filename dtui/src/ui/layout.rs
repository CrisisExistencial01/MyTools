use ratatui::layout::{Constraint, Direction, Layout, Rect};

const TABLE_HEADER_ROWS: u16 = 1;
const TABLE_BORDER_ROWS: u16 = 2;
const TABLE_CHROME_ROWS: u16 = TABLE_HEADER_ROWS + TABLE_BORDER_ROWS;
const FILTER_ROW_HEIGHT: u16 = 5;

pub struct LayoutRects {
    pub list: Rect,
    pub details: Option<Rect>,
    pub statusbar: Rect,
    pub container_visible_rows: usize,
    pub filter_bar: Option<Rect>,
}

pub fn build_layout(area: Rect, show_details: bool, show_filter: bool) -> LayoutRects {
    let filter_height = if show_filter { FILTER_ROW_HEIGHT } else { 0 };

    if !show_details {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let list_area = chunks[0];
        let (list, filter_bar) = if show_filter {
            let inner = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(filter_height), Constraint::Min(1)])
                .split(list_area);
            (inner[1], Some(inner[0]))
        } else {
            (list_area, None)
        };

        return LayoutRects {
            list,
            details: None,
            statusbar: chunks[1],
            container_visible_rows: list.height.saturating_sub(TABLE_CHROME_ROWS) as usize,
            filter_bar,
        };
    }

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical[0]);

    let list_area = horizontal[0];
    let (list, filter_bar) = if show_filter {
        let inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(filter_height), Constraint::Min(1)])
            .split(list_area);
        (inner[1], Some(inner[0]))
    } else {
        (list_area, None)
    };

    LayoutRects {
        list,
        details: Some(horizontal[1]),
        statusbar: vertical[1],
        container_visible_rows: list.height.saturating_sub(TABLE_CHROME_ROWS) as usize,
        filter_bar,
    }
}
