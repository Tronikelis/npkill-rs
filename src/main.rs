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
pub struct AppState {
    pub folders: Vec<Folder>,
    pub list_state: ListState,
}

type AppStateArc = Arc<Mutex<AppState>>;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    println!("Recursively searching for node_modules folders");

    let app_state = Arc::new(Mutex::new(AppState {
        folders: find_target_folders(".", "node_modules"),
        list_state: ListState::default(),
    }));

    list_state_listen(Arc::clone(&app_state));

    loop {
        let app_state = Arc::clone(&app_state);

        if app_state.lock().unwrap().folders.len() <= 0 {
            println!("No node_modules left, the 🦀 did its job");
            return Ok(());
        }

        terminal.draw(|frame| {
            let size = frame.size();
            let mut app_state = app_state.lock().unwrap();

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
                .block(Block::default().title("npkill-rs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            frame.render_stateful_widget(widget, size, &mut app_state.list_state);
        })?;

        thread::sleep(Duration::from_millis(12));
    }
}
