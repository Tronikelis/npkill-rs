#![allow(clippy::needless_return)]

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{backend::CrosstermBackend, widgets::ListState, Terminal};
use spinoff::{spinners, Spinner};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod render;
use render::main::AppRenderer;

mod utils;
use utils::{
    search::{find_target_folders, Folder},
    state::list_state_listen,
};

#[derive(Debug, Clone)]
pub enum Status {
    Kmr,
    Deleting,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub folders: Vec<Folder>,
    pub list_state: ListState,
    pub status: Status,
    pub errors: Option<String>,
}

impl AppState {
    fn not_deleting_folders(&self) -> Vec<&Folder> {
        return self
            .folders
            .iter()
            .filter(|folder| !folder.deleting)
            .collect();
    }
}

type AppStateArc = Arc<Mutex<AppState>>;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let spinner = Spinner::new(
        spinners::Dots,
        "Recursively searching for node_modules folders",
        spinoff::Color::White,
    );

    let app_state = Arc::new(Mutex::new(AppState {
        folders: find_target_folders(".", "node_modules"),
        list_state: {
            let mut list_state = ListState::default();
            list_state.select(Some(0));
            list_state
        },
        status: Status::Kmr,
        errors: None,
    }));

    spinner.stop();
    terminal.clear()?;
    enable_raw_mode()?;

    list_state_listen(Arc::clone(&app_state));

    loop {
        let app_state = Arc::clone(&app_state);

        if app_state.lock().unwrap().folders.is_empty() {
            terminal.clear()?;
            disable_raw_mode()?;

            println!("No node_modules left, the 🦀 did its job");
            return Ok(());
        }

        terminal.draw(|frame| {
            let app_state = app_state.lock().unwrap();
            let mut app_renderer = AppRenderer::new(frame, app_state);

            app_renderer.render_header();
            app_renderer.render_errors();
            app_renderer.render_list();
        })?;

        thread::sleep(Duration::from_millis(12));
    }
}
