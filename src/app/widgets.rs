use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{
    App, CurrentWindow,
    render::{draw_forum_categories, draw_forum_category, draw_forum_thread, draw_homepage},
};

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.window_stack.last().unwrap().render(area, buf);
    }
}

impl Widget for &CurrentWindow {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            CurrentWindow::Homepage => draw_homepage(area, buf),
            CurrentWindow::AllForumCategories => draw_forum_categories(area, buf),
            CurrentWindow::ForumCategory(thread_listing) => {
                draw_forum_category(area, buf, thread_listing)
            }
            CurrentWindow::ForumThread(thread_view) => draw_forum_thread(area, buf, thread_view),
        }
    }
}
