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

pub enum AppEvent {
    KeyUp,
    KeyDown,
    KeyEnter,
}

pub fn thread_event_listen<T>(callback: T) -> JoinHandle<()>
where
    T: Fn(AppEvent) -> () + Send + Sync + 'static,
{
    let join_handle = thread::spawn({
        move || loop {
            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Release,
                    ..
                }) => callback(AppEvent::KeyUp),

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Release,
                    ..
                }) => callback(AppEvent::KeyDown),

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Release,
                    ..
                }) => callback(AppEvent::KeyEnter),

                _ => (),
            };
        }
    });

    return join_handle;
}
