use std::{
    collections::BTreeSet,
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use subscan::{
    enums::{cache::CacheFilter, dispatchers::SubscanModuleDispatcher},
    error::ModuleErrorKind::UrlParse,
    modules::{engines::google::Google, integrations::alienvault::AlienVault},
    pools::module::SubscanModulePool,
    types::{core::SubscanModule, filters::ModuleNameFilter, result::item::PoolResultItem},
};

use crate::common::{
    constants::{LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::{funcs, resolver::MockResolver},
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn submit_test() {
    let resolver = MockResolver::default_boxed();

    let mut dispatcher = Google::dispatcher();

    funcs::wrap_module_url(&mut dispatcher, &stubr.path("/search"));

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), 1, resolver, CacheFilter::default());

    let item = PoolResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap())),
    };

    assert!(pool.clone().is_empty().await);

    pool.clone().start(&vec![google]).await;

    assert_eq!(pool.clone().len().await, 0);
    assert_eq!(pool.result().await.items, [item].into());
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn result_test() {
    let resolver = MockResolver::default_boxed();

    let mut dispatcher = Google::dispatcher();

    funcs::wrap_module_url(&mut dispatcher, &stubr.path("/search"));

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), 1, resolver, CacheFilter::NoFilter);

    pool.clone().start(&vec![google]).await;

    let binding = pool.result().await;
    let result = binding.items.first();

    assert!(result.is_some());
    assert!(result.unwrap().ip.is_some());

    assert_eq!(result.unwrap().subdomain, TEST_BAR_SUBDOMAIN);
    assert_eq!(result.unwrap().ip.unwrap().to_string(), LOCAL_HOST);
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn result_test_with_filter() {
    let resolver = MockResolver::default_boxed();

    let filter = CacheFilter::FilterByName(ModuleNameFilter {
        valids: vec!["google".to_string()],
        invalids: vec!["alienvault".to_string()],
    });

    let mut google_dispatcher = Google::dispatcher();
    let mut alienvault_dispatcher = AlienVault::dispatcher();

    funcs::wrap_module_url(&mut google_dispatcher, &stubr.path("/search"));
    funcs::wrap_module_url(&mut alienvault_dispatcher, &stubr.path("/alienvault"));

    let google = SubscanModule::from(google_dispatcher);
    let alienvault = SubscanModule::from(alienvault_dispatcher);

    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), 1, resolver, filter);

    pool.clone().start(&vec![google, alienvault]).await;

    let binding = pool.result().await;
    let result = binding.items.first();

    assert!(result.is_some());
    assert!(result.unwrap().ip.is_some());

    assert_eq!(binding.items.len(), 1);
    assert_eq!(result.unwrap().subdomain, TEST_BAR_SUBDOMAIN);
    assert_eq!(result.unwrap().ip.unwrap().to_string(), LOCAL_HOST);
}

#[tokio::test]
async fn result_test_with_error() {
    let resolver = MockResolver::default_boxed();

    let mut dispatcher = AlienVault::dispatcher();

    if let SubscanModuleDispatcher::GenericIntegrationModule(ref mut alienvault) = dispatcher {
        alienvault.funcs.url = Box::new(|_| "invalid-url".to_string());
    }

    let alienvault = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), 1, resolver, CacheFilter::NoFilter);

    pool.clone().start(&vec![alienvault]).await;

    let result = pool.result().await;
    let stat = result.statistics.module.first();

    assert!(stat.is_some());

    assert_eq!(result.statistics.module.len(), 1);
    assert_eq!(stat.unwrap().status, UrlParse.into());
    assert_eq!(result.items, BTreeSet::new());
}
