use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{self, Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::{
    io,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

enum Events {
    KeyUp,
    KeyDown,
    KeyEnter,
}

pub fn thread_event_listen(state: Arc<Mutex<ListState>>) -> JoinHandle<()> {
    let join_handle = thread::spawn({
        let state = Arc::clone(&state);
        move || loop {
            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Release,
                    ..
                }) => {
                    let mut state = state.lock().unwrap();
                    let selected = state.selected().unwrap();
                    state.select(Some(selected.clamp(1, 2) - 1));
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Release,
                    ..
                }) => {
                    let mut state = state.lock().unwrap();
                    let selected = state.selected().unwrap();
                    state.select(Some(selected.clamp(0, 1) + 1));
                }
                _ => (),
            };
        }
    });

    return join_handle;
}
