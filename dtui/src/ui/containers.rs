use crate::app::{DomainState, ViewState};
use crate::domain::PortMapping;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;

pub fn render_containers(
    frame: &mut Frame,
    domain: &DomainState,
    view: &ViewState,
    area: Rect,
    filtered: &[usize],
) {
    let visible_rows = view.viewport.visible_rows;
    let total = filtered.len();
    let offset = calculate_offset(view.scroll_offset, total, visible_rows);

    let visible_indices = get_visible_indices(filtered, offset, visible_rows);
    let rows = build_container_rows(domain, view.selected_container, &visible_indices, offset);

    let table = Table::new(rows, column_widths())
        .header(build_header())
        .block(build_table_block(view.selected_container, total, visible_rows));

    frame.render_widget(table, area);
}

pub fn render_filter_bar(frame: &mut Frame, view: &ViewState, area: Rect) {
    let para = build_filter_paragraph(&view.filter.text);
    frame.render_widget(para, area);
}

pub fn calculate_offset(scroll_offset: usize, total: usize, visible_rows: usize) -> usize {
    scroll_offset.min(total.saturating_sub(visible_rows))
}

pub fn get_visible_indices(filtered: &[usize], offset: usize, visible_rows: usize) -> Vec<usize> {
    filtered
        .iter()
        .skip(offset)
        .take(visible_rows)
        .copied()
        .collect()
}

pub fn build_container_rows<'a>(
    domain: &'a DomainState,
    selected_container: Option<usize>,
    visible_indices: &[usize],
    offset: usize,
) -> Vec<Row<'a>> {
    visible_indices
        .iter()
        .enumerate()
        .map(|(i, &actual_idx)| {
            let c = &domain.containers[actual_idx];
            let is_selected = selected_container == Some(offset + i);
            let style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(c.id.as_str()),
                Cell::from(c.name.as_str()),
                Cell::from(c.image.as_str()),
                Cell::from(c.status.as_str()),
                Cell::from(format_ports(&c.ports)),
            ])
            .style(style)
        })
        .collect()
}

pub fn build_table_block(selected: Option<usize>, total: usize, visible_rows: usize) -> Block<'static> {
    Block::default()
        .title(build_title(selected, total, visible_rows))
        .borders(Borders::ALL)
}

pub fn build_header() -> Row<'static> {
    Row::new(vec!["ID", "Name", "Image", "Status", "Ports"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
}

pub fn format_ports(ports: &[PortMapping]) -> String {
    ports
        .iter()
        .map(|p| format!("{}:{}->{}", p.host_ip, p.host_port, p.container_port))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn build_title(selected: Option<usize>, total: usize, visible_rows: usize) -> String {
    if total > visible_rows {
        let pos = selected.map(|s| s + 1).unwrap_or(0);
        format!(" Containers ({pos}/{total}) ")
    } else {
        " Containers ".to_string()
    }
}

pub fn column_widths() -> [Constraint; 5] {
    [
        Constraint::Length(14),
        Constraint::Length(20),
        Constraint::Length(25),
        Constraint::Length(20),
        Constraint::Min(20),
    ]
}

pub fn build_filter_paragraph(filter_text: &str) -> Paragraph<'static> {
    let text = build_filter_text(filter_text);
    Paragraph::new(text)
        .block(build_filter_block())
        .style(Style::default().fg(Color::White))
}

pub fn build_filter_text(filter_text: &str) -> String {
    let cursor = "▌";
    format!("  🔍  {}{}", filter_text, cursor)
}

pub fn build_filter_block() -> Block<'static> {
    Block::default()
        .title(" Filter (Esc/Enter to close) ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
}
