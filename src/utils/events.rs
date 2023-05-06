use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

use std::thread::{self, JoinHandle};

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
