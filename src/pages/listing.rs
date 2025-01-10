use scraper::{ElementRef, Html, Selector};


#[derive(Debug)]
pub struct ThreadListing {
    pub threads: Vec<ThreadView>
}

#[derive(Debug)]
pub struct ThreadView {
    pub name: String,
    pub author: String,
    pub url: String
}

impl ThreadListing {
    pub fn from_html(document: Html) -> ThreadListing {
        let mut threads = Vec::new();

        let thread_view_selector = Selector::parse("
        div.structItemContainer 
        > div.structItemContainer-group.js-threadList 
        > div.structItem.structItem--thread").unwrap();

        for element in document.select(&thread_view_selector) {
            threads.push(ThreadView::from_div(element));
        }

        ThreadListing { threads }
    }
}

impl ThreadView {
    pub fn from_div(element: ElementRef<'_>) -> ThreadView {
        let name_selector = Selector::parse("
        div.structItem--thread 
        > div.structItem-cell--main 
        > div.structItem-title 
        > a").unwrap();

        ThreadView {
            name: element.select(&name_selector).nth(0).unwrap().inner_html().trim().to_string(),
            author: element.attr("data-author").unwrap().to_string(),
            url: element.select(&name_selector).nth(0).unwrap().attr("href").unwrap().to_string()
        }

    }
}