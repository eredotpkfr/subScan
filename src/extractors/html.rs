use crate::{
    enums::Content, extractors::regex::RegexExtractor,
    interfaces::extractor::SubdomainExtractorInterface, types::core::Subdomain,
};
use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeSet;

/// This object compatible with [`SubdomainExtractorInterface`]
/// and it uses `extract` method to extract subdomain addresses
/// from inner text by given `XPath` or `CSS` selector
#[derive(Default)]
pub struct HTMLExtractor {
    selector: String,
    removes: Vec<String>,
    regextractor: RegexExtractor,
}

impl HTMLExtractor {
    /// Creates a new [`HTMLExtractor`] instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::extractors::html::HTMLExtractor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let selector = String::from("div > a");
    ///     let removes = vec!["<br>".to_string()];
    ///
    ///     let extractor = HTMLExtractor::new(selector, removes);
    ///
    ///     // do something with extractor instance
    /// }
    /// ```
    pub fn new(selector: String, removes: Vec<String>) -> Self {
        Self {
            selector,
            removes,
            regextractor: RegexExtractor::default(),
        }
    }
}

#[async_trait]
impl SubdomainExtractorInterface for HTMLExtractor {
    /// Main extraction method to extract subdomains from
    /// given any HTML content by given `XPath` or `CSS` selectors
    ///
    /// # Panics
    ///
    /// When selector is miss-configured
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::extractors::html::HTMLExtractor;
    /// use subscan::interfaces::extractor::SubdomainExtractorInterface;
    /// use subscan::types::core::Subdomain;
    /// use subscan::enums::Content;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let html = Content::from("<div><a>bar.foo.com</a></div>");
    ///     let selector = String::from("div > a");
    ///
    ///     let extractor = HTMLExtractor::new(selector, vec![]);
    ///     let result = extractor.extract(html, "foo.com").await;
    ///
    ///     assert_eq!(result, [Subdomain::from("bar.foo.com")].into());
    /// }
    /// ```
    async fn extract(&self, content: Content, domain: &str) -> BTreeSet<Subdomain> {
        let document = Html::parse_document(&content.as_string());
        let selector = Selector::parse(&self.selector).unwrap();
        let selected = document.select(&selector);

        let remove = |item: ElementRef| {
            let mut text = item.inner_html();

            self.removes.iter().for_each(|element| {
                text = text.replace(element, "");
            });

            text
        };

        let extract = |item| self.regextractor.extract_one(item, domain);

        selected.map(remove).filter_map(extract).collect()
    }
}
