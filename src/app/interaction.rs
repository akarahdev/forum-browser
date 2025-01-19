use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::{App, CurrentWindow};

impl App {
    pub fn handle_events(&mut self) {
        match event::poll(Duration::from_secs(1)) {
            Ok(event_present) => {
                if event_present {
                    self.read_polled_event();
                }
            }
            Err(_) => {}
        }
    }

    pub fn read_polled_event(&mut self) {
        match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_press(key_event);
            }
            _ => {}
        }
    }

    pub fn handle_key_press(&mut self, event: KeyEvent) {
        let window = self.window_stack.last().unwrap();
        match window {
            CurrentWindow::Homepage => {
                if event.code == KeyCode::Esc {
                    ratatui::restore();
                    std::process::exit(0);
                }
                if event.code == KeyCode::Char('f') {
                    self.window_stack.push(CurrentWindow::AllForumCategories);
                }
            }
            CurrentWindow::AllForumCategories => {
                if event.code == KeyCode::Esc {
                    self.window_stack.pop();
                }
            }
            CurrentWindow::ForumCategory(_thread_listing) => {}
            CurrentWindow::ForumThread(_thread_view) => {}
        }
    }
}

impl CurrentWindow {
    pub fn handle_key_press(&mut self, _app: &mut App, _event: KeyEvent) {}
}
