#![allow(clippy::needless_return)]

use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod utils;
use utils::{
    search::{find_target_folders, Folder},
    state::list_state_listen,
};

#[derive(Debug, Clone)]
struct AppState<T> {
    folders: Vec<Folder>,
    state: Arc<Mutex<T>>,
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_state = AppState {
        folders: find_target_folders(".", "node_modules"),
        state: Arc::new(Mutex::new(ListState::default())),
    };

    list_state_listen(Arc::clone(&app_state.state), app_state.folders.len());

    loop {
        let state = Arc::clone(&app_state.state);
        terminal.draw(|frame| {
            let size = frame.size();

            let items: Vec<_> = app_state
                .folders
                .iter()
                .map(|folder| {
                    ListItem::new(format!(
                        "{} -> {}",
                        folder.path.clone(),
                        folder
                            .size
                            .map(|x| (x as f64 / 1e6).to_string() + " MB")
                            .unwrap_or("unknown".to_string())
                    ))
                })
                .collect();

            let widget = List::new(items)
                .block(
                    Block::default()
                        .title("node module paths")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            let mut state = state.lock().unwrap();
            frame.render_stateful_widget(widget, size, &mut state);
        })?;

        thread::sleep(Duration::from_millis(16));
    }
}
