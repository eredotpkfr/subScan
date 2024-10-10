use super::core::Subdomain;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

/// Inner extract method type definition for [`JSONExtractor`](crate::extractors::json::JSONExtractor)
/// In summary it takes a [`Value`] as a parameter and parse subdomains
pub type InnerExtractFunc = Box<dyn Fn(Value, String) -> BTreeSet<Subdomain> + Sync + Send>;
/// Function definition type, [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this type to define method that gets query URL
pub type GetQueryUrlFunc = Box<dyn Fn(&str) -> String + Sync + Send>;
/// Function definition type, [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses this type to define function that gets next query URL to fetch API fully
pub type GetNextUrlFunc = Box<dyn Fn(Url, Value) -> Option<Url> + Sync + Send>;