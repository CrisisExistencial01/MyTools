use crate::app::App;
use crate::config::Config;
use crate::docker::BollardClient;
use crate::domain::{AppError, AppEvent};
use crate::tui::Tui;
use std::sync::mpsc::{Receiver, Sender};

pub struct AppContext<'a> {
    pub tui: &'a mut Tui,
    pub app: &'a mut App,
    pub config: &'a Config,
    pub docker: BollardClient,
    pub event_tx: Sender<AppEvent>,
    pub event_rx: &'a Receiver<AppEvent>,
}

impl<'a> AppContext<'a> {
    pub fn new(
        tui: &'a mut Tui,
        app: &'a mut App,
        config: &'a Config,
        docker: BollardClient,
        event_tx: Sender<AppEvent>,
        event_rx: &'a Receiver<AppEvent>,
    ) -> Self {
        AppContext {
            tui,
            app,
            config,
            docker,
            event_tx,
            event_rx,
        }
    }
}
