use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::forums::listing::{ThreadListing, ThreadView};

use super::App;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            self.handle_events();
            self.time_alive += 1;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

pub fn draw_homepage(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .title(Line::from("Forum Browser").bold().centered())
        .border_set(border::ROUNDED);

    let text = Paragraph::new("[F] All Forums\n[Esc] Quit Forums").block(block);

    text.render(area, buf);
}

pub fn draw_forum_categories(_area: Rect, _buf: &mut Buffer) {}

pub fn draw_forum_category(_area: Rect, _buf: &mut Buffer, _thread_listing: &ThreadListing) {}
pub fn draw_forum_thread(_area: Rect, _buf: &mut Buffer, _thread: &ThreadView) {}
