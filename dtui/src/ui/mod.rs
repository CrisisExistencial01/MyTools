use crate::app::{DomainState, ViewState};
use crate::config::Config;
use crate::domain::Panel;
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
    let show_filter = view.filter.open && view.active_panel == Panel::Containers;
    let rects = layout::build_layout(frame.area(), show_details, show_filter);
    view.viewport.visible_rows = rects.container_visible_rows;

    let filtered = view.filtered_indices(&domain.containers);

    match view.active_panel {
        Panel::Containers => {
            // Filter bar (above table)
            if let Some(filter_area) = rects.filter_bar {
                containers::render_filter_bar(frame, view, filter_area);
            }

            containers::render_containers(frame, domain, view, rects.list, &filtered);

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
