use crate::app::ViewState;
use crate::docker::ContainerAction;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table};
use ratatui::Frame;

#[derive(Debug, Clone)]
pub struct PaletteEntry {
    pub key: String,
    pub action: ContainerAction,
    pub desc: String,
}

pub fn palette_commands() -> Vec<PaletteEntry> {
    vec![
        PaletteEntry {
            key: "s".into(),
            action: ContainerAction::Start,
            desc: "Start container".into(),
        },
        PaletteEntry {
            key: "x".into(),
            action: ContainerAction::Stop,
            desc: "Stop container".into(),
        },
        PaletteEntry {
            key: "r".into(),
            action: ContainerAction::Restart,
            desc: "Restart container".into(),
        },
        PaletteEntry {
            key: "p".into(),
            action: ContainerAction::Pause,
            desc: "Pause container".into(),
        },
        PaletteEntry {
            key: "u".into(),
            action: ContainerAction::Unpause,
            desc: "Unpause container".into(),
        },
        PaletteEntry {
            key: "k".into(),
            action: ContainerAction::Kill,
            desc: "Kill container".into(),
        },
    ]
}

pub fn render_palette(frame: &mut Frame, view: &ViewState, area: Rect) {
    // Clear + solid background
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL),
        area,
    );

    let all = palette_commands();
    let filtered: Vec<_> = all
        .iter()
        .filter(|e| {
            e.key.contains(&view.palette.filter)
                || e.desc.to_lowercase().contains(&view.palette.filter)
        })
        .collect();

    let header = Row::new(vec!["Key", "Action", "Description"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = filtered
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let style = if i == view.palette.cursor {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(e.key.clone()),
                Cell::from(e.desc.clone()),
                Cell::from(""),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(6),
        Constraint::Length(20),
        Constraint::Min(20),
    ];
    let table = Table::new(rows, widths).header(header).block(
        Block::default()
            .title(format!(" Actions ({}) ", view.palette.filter))
            .borders(Borders::ALL),
    );

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    frame.render_widget(table, inner);
}
