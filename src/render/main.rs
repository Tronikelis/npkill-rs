use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::sync::MutexGuard;

use crate::{AppState, Status};

type MutexAppState<'a> = MutexGuard<'a, AppState>;

pub struct AppRenderer<'a, 'b, 'c, T>
where
    T: Backend,
{
    frame: &'c mut Frame<'a, T>,
    app_state: MutexAppState<'b>,
    global_layout: GlobalLayout,
}

#[derive(Debug, Clone)]
struct GlobalLayout {
    header: Rect,
    body: Rect,
}

impl<'a, 'b, 'c, T> AppRenderer<'a, 'b, 'c, T>
where
    T: Backend,
{
    pub fn new(frame: &'c mut Frame<'a, T>, app_state: MutexAppState<'b>) -> Self {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .constraints([Constraint::Min(5), Constraint::Min(0)])
            .split(frame.size());

        let global_layout = GlobalLayout {
            header: layout[0],
            body: layout[1],
        };

        return AppRenderer {
            frame,
            app_state,
            global_layout,
        };
    }

    pub fn render_header(&mut self) {
        let area = self.global_layout.header;
        let container = Block::default().borders(Borders::all());

        let text_layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(2)
            .split(area);

        let text_layout = text_layout[0];

        let branding = Paragraph::new("npkill-rs 🦀")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        let status_text = "status: ".to_string();
        let status_text = match self.app_state.status {
            Status::Kmr => status_text + " kmr 🇱🇹 (chilling)",
            Status::Deleting => status_text + " deleting files, wait!",
        };

        let status = Paragraph::new(status_text).alignment(Alignment::Left);

        self.frame.render_widget(container, area);
        self.frame.render_widget(status, text_layout);
        self.frame.render_widget(branding, text_layout);
    }

    pub fn render_body(&mut self) {
        let area = self.global_layout.body;
        let container = Block::default().borders(Borders::all());

        let items: Vec<_> = self
            .app_state
            .not_deleting_folders()
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
            .block(container)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Red))
            .highlight_symbol(">> ");

        self.frame
            .render_stateful_widget(widget, area, &mut self.app_state.list_state);
    }
}
