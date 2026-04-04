use crate::app::{DomainState, ViewState};
use crate::domain::PortMapping;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::Frame;

pub fn render_containers(frame: &mut Frame, domain: &DomainState, view: &ViewState, area: Rect) {
    let visible_rows = view.viewport.visible_rows;
    let total = domain.container_count();
    let offset = view.scroll_offset.min(total.saturating_sub(visible_rows));
    let end = (offset + visible_rows).min(total);
    let visible = &domain.containers[offset..end];

    let table = Table::new(
        build_rows(visible, offset, view.selected_container),
        column_widths(),
    )
    .header(build_header())
    .block(
        Block::default()
            .title(build_title(view.selected_container, total, visible_rows))
            .borders(Borders::ALL),
    );

    frame.render_widget(table, area);
}

fn build_header() -> Row<'static> {
    Row::new(vec!["ID", "Name", "Image", "Status", "Ports"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
}

fn build_rows<'a>(
    containers: &'a [crate::domain::Container],
    offset: usize,
    selected: Option<usize>,
) -> Vec<Row<'a>> {
    containers
        .iter()
        .enumerate()
        .map(|(i, c)| container_to_row(c, offset + i, selected))
        .collect()
}

fn container_to_row<'a>(
    container: &'a crate::domain::Container,
    index: usize,
    selected: Option<usize>,
) -> Row<'a> {
    let style = if selected == Some(index) {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    } else {
        Style::default()
    };

    Row::new(vec![
        Cell::from(container.id.as_str()),
        Cell::from(container.name.as_str()),
        Cell::from(container.image.as_str()),
        Cell::from(container.status.as_str()),
        Cell::from(format_ports(&container.ports)),
    ])
    .style(style)
}

fn format_ports(ports: &[PortMapping]) -> String {
    ports
        .iter()
        .map(|p| format!("{}:{}->{}", p.host_ip, p.host_port, p.container_port))
        .collect::<Vec<_>>()
        .join(", ")
}

fn build_title(selected: Option<usize>, total: usize, visible_rows: usize) -> String {
    if total > visible_rows {
        let pos = selected.map(|s| s + 1).unwrap_or(0);
        format!(" Containers ({pos}/{total}) ")
    } else {
        " Containers ".to_string()
    }
}

fn column_widths() -> [Constraint; 5] {
    [
        Constraint::Length(14),
        Constraint::Length(20),
        Constraint::Length(25),
        Constraint::Length(20),
        Constraint::Min(20),
    ]
}
