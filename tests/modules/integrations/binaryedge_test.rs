use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mocks::wrap_url_with_mock_func,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::binaryedge::{self, BINARYEDGE_MODULE_NAME, BINARYEDGE_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/binaryedge.json")]
async fn binaryedge_run_test() {
    let mut binaryedge = binaryedge::Binaryedge::new();
    let (env_name, _) = binaryedge.fetch_apikey().await;

    env::set_var(&env_name, "binaryedge-api-key");

    binaryedge.url = wrap_url_with_mock_func(stubr.path("/binaryedge").as_str());

    let result = binaryedge.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(binaryedge.name().await, BINARYEDGE_MODULE_NAME);
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = binaryedge::Binaryedge::get_query_url(TEST_DOMAIN);
    let expected = format!("{BINARYEDGE_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let content = "{\"events\": [\"bar.foo.com\"]}";
    let json = serde_json::from_str(content).unwrap();

    let extracted = binaryedge::Binaryedge::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = binaryedge::Binaryedge::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
