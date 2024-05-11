use crate::interfaces::requester::RequesterInterface;
use async_trait::async_trait;
use headless_chrome::browser::default_executable;
use headless_chrome::browser::LaunchOptions;
use headless_chrome::{Browser, Tab};
use reqwest::{Client, Request, RequestBuilder};
use reqwest::{Method, Url};
use std::sync::Arc;

#[derive(Clone)]
pub struct ChromeBrowser {
    browser: Browser,
    client: Client,
}

impl ChromeBrowser {
    pub fn new() -> Self {
        let builder = LaunchOptions::default_builder()
            .path(Some(default_executable().unwrap()))
            .headless(true)
            .sandbox(false)
            .build()
            .unwrap();

        ChromeBrowser {
            browser: Browser::new(builder).unwrap(),
            client: Client::new(),
        }
    }
}

#[async_trait(?Send)]
impl RequesterInterface for ChromeBrowser {
    fn request(&self, method: Method, url: Url) -> RequestBuilder {
        self.client.request(method, url)
    }

    async fn get(&self, request: Request) -> String {
        let tab = self.browser.new_tab().unwrap();
        let _ = tab.navigate_to(request.url().as_str());
        tab.wait_until_navigated().unwrap();

        tab.get_content().unwrap()
    }
}