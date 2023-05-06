use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::ListState;

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub enum Key {
    Up,
    Down,
    Enter,
}

pub fn thread_event_listen<T>(callback: T) -> JoinHandle<()>
where
    T: Fn(Key) + Send + Sync + 'static,
{
    return thread::spawn({
        move || loop {
            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Release,
                    ..
                }) => callback(Key::Up),

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Release,
                    ..
                }) => callback(Key::Down),

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Release,
                    ..
                }) => callback(Key::Enter),

                _ => (),
            };
        }
    });
}

pub fn list_state_listen(list_state: Arc<Mutex<ListState>>, list_len: usize) -> JoinHandle<()> {
    enum Dir {
        Up,
        Down,
    }

    let move_list = move |dir: Dir| {
        let mut state = list_state.lock().unwrap();
        let selected: i64 = state.selected().unwrap_or(0).try_into().unwrap();
        let list_len: i64 = list_len.try_into().unwrap_or(0);

        let new = match dir {
            Dir::Up => (selected - 1).clamp(0, list_len - 1),
            Dir::Down => (selected + 1).clamp(0, list_len - 1),
        };

        let new: usize = new.try_into().unwrap_or(0);
        state.select(Some(new));
    };

    return thread_event_listen({
        move |event| match event {
            Key::Down => move_list(Dir::Down),
            Key::Up => move_list(Dir::Up),
            Key::Enter => {
                println!("Pressed ENTER");
            }
        }
    });
}
