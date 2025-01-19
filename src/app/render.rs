use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::forums::{
    listing::{ThreadListing, ThreadView},
    thread::Thread,
};

use super::App;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.clear().unwrap();
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

pub fn draw_forum_categories(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .title(Line::from("Forum Browser").bold().centered())
        .border_set(border::ROUNDED);

    let text = Paragraph::new("[O] Off Topic\n[S] SkyBlock General Discussion")
        .block(block)
        .render(area, buf);
}

pub fn draw_forum_category(
    area: Rect,
    buf: &mut Buffer,
    thread_listing: &ThreadListing,
    scrolled_by: i32,
) {
    let block = Block::bordered()
        .title(Line::from("Forum Browser").bold().centered())
        .title_bottom(
            Line::from("[W] Go Up   [S] Go Down   [Esc] Leave")
                .bold()
                .centered(),
        )
        .border_set(border::ROUNDED);

    let mut str = String::with_capacity(1024);
    let keys = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "z", "x", "c", "v", "b", "n", "m", "f",
        "g", "h",
    ];

    let mut idx: usize = 0;
    for thread in &thread_listing.threads {
        idx += 1;
        if (scrolled_by as usize) < idx {
            str.push_str(&format!(
                "[{}] {}\nby {}\n",
                keys[idx - 1],
                thread.name,
                thread.author
            ));
        }
    }

    Paragraph::new(str).block(block).render(area, buf);
}

pub fn draw_forum_thread(area: Rect, buf: &mut Buffer, thread: &Thread, reply_idx: i32) {
    let block = Block::bordered()
        .title(
            Line::from(format!("Thread | {} @ {}", thread.title, thread.url))
                .bold()
                .centered(),
        )
        .title_bottom(
            Line::from("[A] Last Post   [D] Next Post   [Esc] Leave")
                .bold()
                .centered(),
        )
        .border_set(border::ROUNDED);

    let reply_idx = reply_idx % 20;
    match thread.posts.get(reply_idx as usize) {
        Some(reply) => {
            Paragraph::new(reply.contents.clone())
                .block(block)
                .render(area, buf);
        }
        None => {
            Paragraph::new(format!(
                "reply does not exist @ {}\n{:?}",
                reply_idx, thread.posts
            ))
            .block(block)
            .render(area, buf);
        }
    }
}
