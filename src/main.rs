use forums::{listing::ThreadListing, thread::Thread};
use reqwest::{Method, blocking::Client};
use scraper::Html;

pub mod app;
pub mod forums;

fn main() {
    println!("Hello, world!");

    let client = Client::new();

    let rq = client
        .request(Method::GET, "https://hypixel.net/forums/off-topic.2")
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        )
        .build()
        .unwrap();
    let rsp = client.execute(rq).unwrap();

    let html = rsp.text().unwrap();
    let output = ThreadListing::from_html(Html::parse_document(&html));
    println!("{:#?}", output);

    let first_thread_url = &output.threads[0].url;

    let rq = client
        .request(
            Method::GET,
            format!("https://hypixel.net{}", first_thread_url),
        )
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        )
        .build()
        .unwrap();
    let rsp = client.execute(rq).unwrap();
    let html = rsp.text().unwrap();
    let output = Thread::from_html(Html::parse_document(&html));
    println!("{:#?}", output);
}
