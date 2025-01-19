use reqwest::{Method, blocking::Client};
use scraper::Html;

pub struct ForumsClient {
    pub inner: Client,
}

impl ForumsClient {
    pub fn request_page(&self, page: impl Into<String>) -> Html {
        let rq = self
            .inner
            .request(Method::GET, page.into())
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
            )
            .header(
                "Cookie",
                "xfNew_consent=%5B%22_third_party%22%2C%22optional%22%5D",
            )
            .build()
            .unwrap();
        let rsp = self.inner.execute(rq).unwrap();

        let html = rsp.text().unwrap();
        Html::parse_document(&html)
    }
}
