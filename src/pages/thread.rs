use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct Thread {
    pub title: String,
    pub posts: Vec<ThreadPost>
}

#[derive(Debug)]
pub struct AuthorBox {
    pub forum_name: String,
    pub mc_name: String
}

#[derive(Debug)]
pub struct ThreadPost {
    pub author: AuthorBox,
    pub contents: String,
    pub post_date: String
}

impl Thread {
    pub fn from_html(html: Html) -> Thread {
        let mut posts = Vec::new();

        let reply_selector = Selector::parse("article.message").unwrap();

        for element in html.select(&reply_selector) {
            posts.push(ThreadPost::from_element(element));
        }

        Thread {
            title: "TODO".to_string(),
            posts
        }
    }
}

impl ThreadPost {
    pub fn from_element(element: ElementRef<'_>) -> ThreadPost {
        let message_content_selector = Selector::parse("
            div.message-inner 
            > div.message-cell.message-cell--main 
            > div.message-main
            > div.message-content
            > div.message-userContent
            > article.message-body 
            > div
            > div.bbWrapper").unwrap();

        let post_time_selector = Selector::parse("
            div.message-inner 
            > div.message-cell--main 
            > div.message-main
            > header.message-attribution
            > ul.message-attribution-main
            > li.u-concealed
            > a
            > time").unwrap();

        ThreadPost {
            contents: element.select(&message_content_selector).nth(0).unwrap().inner_html(),
            author: AuthorBox {
                forum_name: "TODO".to_string(),
                mc_name: "TODO".to_string()
            },
            post_date: element.select(&post_time_selector).nth(0).unwrap().inner_html()
        }
    }
}