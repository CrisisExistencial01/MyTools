use crate::app::{DomainState, ViewState};
use crate::config::Config;
use crate::docker::ContainerAction;
use crate::domain::{AppAction, Panel};
use ratatui::Frame;

mod containers;
mod details;
mod help;
mod layout;
pub mod palette;
mod statusbar;
mod volumes;

pub fn render(frame: &mut Frame, domain: &DomainState, view: &mut ViewState, config: &Config) {
    let show_details = view.show_details && view.active_panel == Panel::Containers;
    let rects = layout::build_layout(frame.area(), show_details);
    view.viewport.visible_rows = rects.container_visible_rows;

    match view.active_panel {
        Panel::Containers => {
            containers::render_containers(frame, domain, view, rects.list);
            if show_details {
                if let Some(da) = rects.details {
                    details::render_details(frame, view, da);
                }
            }
        }
        Panel::Volumes => {
            volumes::render_volumes(frame, rects.list);
        }
    }

    if view.show_help {
        help::render_help(frame, config, frame.area());
    }

    if view.palette.open {
        palette::render_palette(frame, view, frame.area());
    }

    statusbar::render_statusbar(frame, view, rects.statusbar);
}
