use crate::config::Config;
use crate::docker::ContainerAction;
use crate::domain::AppAction;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table};
use ratatui::Frame;

pub fn render_help(frame: &mut Frame, _config: &Config, area: Rect) {
    frame.render_widget(Clear, area);

    frame.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL),
        area,
    );

    let sections = [
        (
            "Navigation",
            vec![
                ("j / k", "Down / Up (cyclic)"),
                ("Enter", "Toggle details panel"),
            ],
        ),
        (
            "Command Palette (/)",
            vec![
                ("s", "Start container"),
                ("x", "Stop container"),
                ("r", "Restart container"),
                ("p", "Pause container"),
                ("u", "Unpause container"),
                ("k", "Kill container"),
            ],
        ),
        (
            "Panels",
            vec![("Tab", "Next panel"), ("Shift+Tab", "Previous panel")],
        ),
        ("App", vec![("?", "Toggle this help"), ("q", "Quit")]),
    ];

    let mut rows = Vec::new();
    for (title, items) in sections {
        rows.push(
            Row::new(vec![Cell::from(title), Cell::from("")]).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        );
        for (key, desc) in items {
            rows.push(Row::new(vec![Cell::from(key), Cell::from(desc)]));
        }
        rows.push(Row::new(vec![Cell::from(""), Cell::from("")])); // spacer
    }

    let widths = [Constraint::Length(18), Constraint::Min(30)];
    let table = Table::new(rows, widths);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    frame.render_widget(table, inner);

    let footer =
        Paragraph::new(" Press ? or Esc to close ").style(Style::default().fg(Color::DarkGray));
    frame.render_widget(
        footer,
        Rect {
            x: area.x,
            y: area.y + area.height - 2,
            width: area.width,
            height: 1,
        },
    );
}
