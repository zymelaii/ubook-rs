use reqwest::{
    header::{HeaderMap, COOKIE},
    Client, Method,
};

pub trait RequestBuilder {
    fn api_prefix(&self) -> &'static str;

    fn default_header(&self) -> crate::Result<HeaderMap>;

    fn default_cookie(&self) -> crate::Result<String>;

    fn request(&self, method: Method, url: &str) -> reqwest::RequestBuilder {
        Client::new()
            .request(method, url)
            .headers(self.default_header().unwrap_or_default())
            .header(COOKIE, self.default_cookie().unwrap_or_default())
    }

    //fn get<U: IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
    //    self.request(Method::GET, url)
    //}

    //fn post<U: IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
    //    self.request(Method::POST, url)
    //}

    fn api_get(&self, url: &str) -> reqwest::RequestBuilder {
        self.request(Method::GET, format!("{}{url}", self.api_prefix()).as_str())
    }

    fn api_post(&self, url: &str) -> reqwest::RequestBuilder {
        self.request(Method::POST, format!("{}{url}", self.api_prefix()).as_str())
    }
}
