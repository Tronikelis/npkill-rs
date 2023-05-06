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
    events::{thread_event_listen, Key},
    search::{find_target_folders, Folder},
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

    thread_event_listen({
        let state = Arc::clone(&app_state.state);
        let folder_len = app_state.folders.len() as i64;

        move |event| match event {
            Key::Down => {
                let mut state = state.lock().unwrap();
                let selected = state.selected().unwrap_or(0) as i64;
                let new = (selected + 1).clamp(0, folder_len - 1);
                state.select(Some(new.try_into().unwrap_or(0)));
            }
            Key::Up => {
                let mut state = state.lock().unwrap();
                let selected = state.selected().unwrap_or(0) as i64;
                let new = (selected - 1).clamp(0, folder_len - 1);
                state.select(Some(new.try_into().unwrap_or(0)));
            }
            Key::Enter => {
                println!("Pressed ENTER");
            }
        }
    });

    loop {
        let state = Arc::clone(&app_state.state);
        terminal.draw(|frame| {
            let size = frame.size();

            let items: Vec<_> = app_state
                .folders
                .iter()
                .map(|folder| ListItem::new(folder.clone().path))
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
