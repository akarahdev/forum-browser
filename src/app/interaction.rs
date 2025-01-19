use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::forums::{
    listing::{ThreadListing, ThreadView},
    thread::Thread,
};

use super::{App, CurrentWindow};

impl App {
    pub fn handle_events(&mut self) {
        match event::poll(Duration::from_secs(10)) {
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
        let window = self.window_stack.last_mut().unwrap();
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
                if event.code == KeyCode::Char('o') {
                    let url = "https://hypixel.net/forums/off-topic.2";
                    let document = self.client.request_page(url);
                    let thread_listing = ThreadListing::from_html(document);
                    self.window_stack.push(CurrentWindow::ForumCategory {
                        thread_listing,
                        scrolled_by: 0,
                    });
                }
            }
            CurrentWindow::ForumCategory {
                #[allow(unused)]
                thread_listing,
                scrolled_by,
            } => {
                if event.code == KeyCode::Esc {
                    self.window_stack.pop();
                    return;
                } else if event.code == KeyCode::Char('w') {
                    *scrolled_by = 0.max(*scrolled_by - 1);
                } else if event.code == KeyCode::Char('s') {
                    *scrolled_by = 20.min(*scrolled_by + 1);
                }

                let keys = [
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "z", "x", "c", "v", "b", "n",
                    "m", "f", "g", "h",
                ]
                .map(|x| x.chars().next().unwrap());
                for idx in 0..=19 {
                    let key = keys[idx];
                    if event.code == KeyCode::Char(key) {
                        let url = thread_listing.clone().threads[idx].url.clone();
                        let url = format!("https://hypixel.net{}", url);
                        self.window_stack.push(CurrentWindow::ForumThread {
                            thread: Thread::from_html(self.client.request_page(url.clone()), url),
                            reply_idx: 0,
                        });
                        return;
                    }
                }
            }
            CurrentWindow::ForumThread { thread, reply_idx } => {
                if event.code == KeyCode::Esc {
                    self.window_stack.pop();
                } else if event.code == KeyCode::Char('a') {
                    *reply_idx = 0.max(*reply_idx - 1);
                } else if event.code == KeyCode::Char('d') {
                    *reply_idx = *reply_idx + 1;
                }
            }
        }
    }
}

impl CurrentWindow {
    pub fn handle_key_press(&mut self, _app: &mut App, _event: KeyEvent) {}
}
