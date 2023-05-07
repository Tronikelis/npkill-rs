use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use if_chain::if_chain;
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{AppStateArc, Status};

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
                    kind: KeyEventKind::Press,
                    ..
                }) => callback(Key::Up),

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    ..
                }) => callback(Key::Down),

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    ..
                }) => callback(Key::Enter),

                _ => (),
            };
        }
    });
}

pub fn list_state_listen(app_state: AppStateArc) -> JoinHandle<()> {
    enum Dir {
        Up,
        Down,
    }

    let move_list = {
        let app_state = Arc::clone(&app_state);
        move |dir: Dir| {
            let mut app_state = app_state.lock().unwrap();

            let list_len: i64 = app_state.folders.len().try_into().unwrap_or(0);
            if list_len <= 0 {
                return;
            }

            let selected: i64 = app_state
                .list_state
                .selected()
                .unwrap_or(0)
                .try_into()
                .unwrap();

            let new = match dir {
                Dir::Up => (selected - 1).clamp(0, list_len - 1),
                Dir::Down => (selected + 1).clamp(0, list_len - 1),
            };

            let new: usize = new.try_into().unwrap_or(0);
            app_state.list_state.select(Some(new));
        }
    };

    let on_enter = {
        let app_state = Arc::clone(&app_state);
        move || {
            let mut app_state_locked = app_state.lock().unwrap();

            let selected = app_state_locked.list_state.selected();

            if_chain! {
                if let Some(selected) = selected;
                if let Some(folder) = app_state_locked.folders.get(selected).cloned();
                then {
                    thread::spawn({
                        let app_state = Arc::clone(&app_state);
                        move || {
                            {
                                app_state.lock().unwrap().status = Status::Deleting;
                            }

                            std::fs::remove_dir_all(folder.path).unwrap();
                            app_state.lock().unwrap().status = Status::Kmr;
                        }
                    });
                    app_state_locked.folders.remove(selected);
                }
            };
        }
    };

    return thread_event_listen({
        move |event| match event {
            Key::Down => move_list(Dir::Down),
            Key::Up => move_list(Dir::Up),
            Key::Enter => on_enter(),
        }
    });
}
