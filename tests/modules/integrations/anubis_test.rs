use std::collections::BTreeSet;

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks::wrap_url_with_mock_func,
};
use serde_json::{self, Value};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::anubis::{self, ANUBIS_MODULE_NAME, ANUBIS_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/anubis.json")]
async fn anubis_run_test() {
    let mut anubis = anubis::Anubis::new();

    anubis.url = wrap_url_with_mock_func(stubr.path("/anubis").as_str());

    let result = anubis.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(anubis.name().await, ANUBIS_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = anubis::Anubis::get_query_url(TEST_DOMAIN);
    let expected = format!("{ANUBIS_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let content = "[\"bar.foo.com\"]";
    let json = serde_json::from_str(content).unwrap();

    let extracted = anubis::Anubis::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = anubis::Anubis::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
