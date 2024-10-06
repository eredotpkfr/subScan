use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const ANUBIS_MODULE_NAME: &str = "anubis";
pub const ANUBIS_URL: &str = "https://jonlu.ca/anubis/subdomains";

/// `Anubis` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                           |
/// |:------------------:|:-------------------------------:|
/// | Module Name        | `anubis`                        |
/// | Doc URL            | <https://jonlu.ca/anubis>       |
/// | Authentication     | [`APIAuthMethod::NoAuth`]       |
/// | Requester          | [`HTTPClient`]                  |
/// | Extractor          | [`JSONExtractor`]               |
pub struct Anubis {}

impl Anubis {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: ANUBIS_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: APIAuthMethod::NoAuth,
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{ANUBIS_URL}/{domain}")
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: String) -> BTreeSet<Subdomain> {
        if let Some(subs) = content.as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            return subs.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
