use crate::{
    enums::RequesterDispatcher, extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient, types::core::Subdomain,
};
use serde_json::Value;
use std::collections::BTreeSet;

/// Anubis API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Anubis {}

const ANUBIS_MODULE_NAME: &str = "Anubis";
const ANUBIS_URL: &str = "https://jonlu.ca/anubis/subdomains/";

impl Anubis {
    /// Create a new [`Anubis`] module instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::integrations::anubis;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let anubis = anubis::Anubis::new();
    ///
    ///     // do something with anubis instance
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: ANUBIS_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    /// Get Anubis query URL from given domain address
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::integrations::anubis;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let url = anubis::Anubis::get_query_url("foo.com".to_string());
    ///     let expected = "https://jonlu.ca/anubis/subdomains/foo.com";
    ///
    ///     assert_eq!(url, expected);
    /// }
    /// ```
    pub fn get_query_url(domain: String) -> String {
        format!("{ANUBIS_URL}{domain}")
    }

    /// JSON parse method to extract subdomains
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::integrations::anubis;
    /// use std::collections::BTreeSet;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = anubis::Anubis::extract(Value::default());
    ///
    ///     assert_eq!(result, BTreeSet::default());
    /// }
    /// ```
    pub fn extract(content: Value) -> BTreeSet<Subdomain> {
        if let Some(subs) = content.as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::default()
        }
    }
}