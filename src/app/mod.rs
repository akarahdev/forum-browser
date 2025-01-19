use crate::forums::listing::{ThreadListing, ThreadView};

pub mod interaction;
pub mod render;
pub mod widgets;

pub struct App {
    pub time_alive: i32,
    pub window_stack: Vec<CurrentWindow>,
    pub login_status: LoginStatus,
}

pub enum CurrentWindow {
    Homepage,
    AllForumCategories,
    ForumCategory(ThreadListing),
    ForumThread(ThreadView),
}

pub enum LoginStatus {
    Guest,
    User { session_token: String },
}
