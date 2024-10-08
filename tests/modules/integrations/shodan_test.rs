use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    funcs::read_stub,
    mocks,
};
use reqwest::Url;
use serde_json::{json, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::shodan::{Shodan, SHODAN_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/shodan.json")]
async fn run_test() {
    let mut shodan = Shodan::dispatcher();
    let (env_name, _) = shodan.fetch_apikey().await;

    env::set_var(&env_name, "shodan-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut shodan, &stubr.path("/shodan"));

    let result = shodan.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = Shodan::get_query_url(TEST_DOMAIN);
    let expected = format!("{SHODAN_URL}/dns/domain/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();

    let mut next = Shodan::get_next_url(url.clone(), Value::Null);
    let mut expected = Url::parse(&format!("{TEST_URL}/?page=2")).unwrap();

    assert!(next.is_none());

    next = Shodan::get_next_url(url.clone(), json!({"more": false}));

    assert!(next.is_none());

    next = Shodan::get_next_url(url.clone(), json!({"more": true}));

    assert_eq!(next.clone().unwrap(), expected);

    next = Shodan::get_next_url(next.unwrap(), json!({"more": true}));
    expected = Url::parse(&format!("{TEST_URL}/?page=3")).unwrap();

    assert_eq!(next.unwrap(), expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/shodan.json")["response"]["jsonBody"].clone();
    let extracted = Shodan::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = Shodan::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}