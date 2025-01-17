use reqwest::Url;

use crate::{
    enums::dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::engine::GenericSearchEngineModule,
    requesters::client::HTTPClient,
    types::core::SubscanModuleCoreComponents,
};

pub const YAHOO_MODULE_NAME: &str = "yahoo";
pub const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
pub const YAHOO_SEARCH_PARAM: &str = "p";
pub const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

/// Yahoo search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                 |
/// |:------------------:|:-------------------------------------:|
/// | Module Name        | `yahoo`                               |
/// | Search URL         | <https://search.yahoo.com/search>     |
/// | Search Param       | `p`                                   |
/// | Subdomain Selector | `ol > li > div > div > h3 > a > span` |
/// | Requester          | [`HTTPClient`]                        |
/// | Extractor          | [`HTMLExtractor`]                     |
/// | Generic            | [`GenericSearchEngineModule`]         |
pub struct Yahoo {}

impl Yahoo {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(YAHOO_SEARCH_URL);

        let selector: String = YAHOO_CITE_TAG.into();
        let removes: Vec<String> = vec!["<b>".into(), "</b>".into()];

        let extractor: HTMLExtractor = HTMLExtractor::new(selector, removes);
        let requester: RequesterDispatcher = HTTPClient::default().into();

        let generic = GenericSearchEngineModule {
            name: YAHOO_MODULE_NAME.into(),
            param: YAHOO_SEARCH_PARAM.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        generic.into()
    }
}
