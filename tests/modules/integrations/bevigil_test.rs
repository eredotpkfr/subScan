use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    mocks::wrap_url_with_mock_func,
};
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::bevigil::{self, BEVIGIL_MODULE_NAME, BEVIGIL_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/bevigil.json")]
async fn bevigil_run_test() {
    env::set_var("SUBSCAN_BEVIGIL_APIKEY", "bevigil-api-key");

    let mut bevigil = bevigil::Bevigil::new();

    bevigil.url = wrap_url_with_mock_func(stubr.path("/bevigil").as_str());

    let result = bevigil.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(bevigil.name().await, BEVIGIL_MODULE_NAME);
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );
}

#[tokio::test]
async fn get_query_url_test() {
    let url = bevigil::Bevigil::get_query_url(TEST_DOMAIN);
    let expected = format!("{BEVIGIL_URL}/{TEST_DOMAIN}/subdomains");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = "{\"subdomains\": [\"bar.foo.com\"]}";

    let extracted = bevigil::Bevigil::extract(serde_json::from_str(json).unwrap());
    let not_extracted = bevigil::Bevigil::extract(Value::default());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.to_string()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}