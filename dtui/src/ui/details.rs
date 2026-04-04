use crate::app::ViewState;
use crate::docker::ContainerDetails;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::Frame;

pub fn render_details(frame: &mut Frame, view: &ViewState, area: Rect) {
    let details = match &view.selected_details {
        Some(d) => d,
        None => {
            use ratatui::widgets::Paragraph;
            let block = Block::default().title(" Details ").borders(Borders::ALL);
            frame.render_widget(
                Paragraph::new("Select a container to see details").block(block),
                area,
            );
            return;
        }
    };

    let uptime_str = format_uptime(details.uptime);
    let ports_str = format_ports(&details.ports);
    let created_str = format_datetime(details.created_at);
    let started_str = format_datetime(details.started_at);
    let state_str = format!("{:?}", details.state);

    let rows = vec![
        detail_row("ID", &details.id),
        detail_row("Name", &details.name),
        detail_row("Image", &details.image),
        detail_row("State", &state_str),
        detail_row("Status", &details.status),
        detail_row("Uptime", &uptime_str),
        detail_row("Ports", &ports_str),
        detail_row("Command", &details.command),
        detail_row("Created", &created_str),
        detail_row("Started", &started_str),
    ];

    let widths = [Constraint::Length(12), Constraint::Min(30)];

    let table =
        Table::new(rows, widths).block(Block::default().title(" Details ").borders(Borders::ALL));

    frame.render_widget(table, area);
}

fn detail_row<'a>(label: &'a str, value: &'a str) -> Row<'a> {
    Row::new(vec![
        Cell::from(label).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(value),
    ])
}

fn format_uptime(uptime: Option<std::time::Duration>) -> String {
    match uptime {
        Some(d) => {
            let secs = d.as_secs();
            let hours = secs / 3600;
            let mins = (secs % 3600) / 60;
            let s = secs % 60;
            format!("{hours}h {mins}m {s}s")
        }
        None => "—".to_string(),
    }
}

fn format_ports(ports: &[crate::domain::PortMapping]) -> String {
    if ports.is_empty() {
        return "—".to_string();
    }
    ports
        .iter()
        .map(|p| format!("{}:{}->{}", p.host_ip, p.host_port, p.container_port))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_datetime(dt: Option<chrono::DateTime<chrono::Utc>>) -> String {
    match dt {
        Some(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "—".to_string(),
    }
}
