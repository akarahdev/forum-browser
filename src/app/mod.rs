use crate::{
    client::ForumsClient,
    forums::{
        listing::{ThreadListing, ThreadView},
        thread::Thread,
    },
};

pub mod interaction;
pub mod render;
pub mod widgets;

pub struct App {
    pub time_alive: i32,
    pub window_stack: Vec<CurrentWindow>,
    pub login_status: LoginStatus,
    pub client: ForumsClient,
}

#[derive(Clone)]
pub enum CurrentWindow {
    Homepage,
    AllForumCategories,
    ForumCategory {
        thread_listing: ThreadListing,
        scrolled_by: i32,
    },
    ForumThread {
        thread: Thread,
        reply_idx: i32,
    },
}

pub enum LoginStatus {
    Guest,
    User { session_token: String },
}
