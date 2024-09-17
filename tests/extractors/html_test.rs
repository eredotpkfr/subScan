use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_testdata,
};
use subscan::extractors::html::HTMLExtractor;
use subscan::interfaces::extractor::SubdomainExtractorInterface;

#[tokio::test]
async fn extract_without_removes() {
    let html = read_testdata("html/subdomains.html");

    let selector = String::from("article > div > a > span:first-child");
    let extractor = HTMLExtractor::new(selector, vec![]);
    let result = extractor.extract(html, TEST_DOMAIN.to_string()).await;

    assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
}

#[tokio::test]
async fn extract_with_removes() {
    let html = read_testdata("html/subdomains-with-removes.html");

    let selector = String::from("article > div > a > span");
    let extractor = HTMLExtractor::new(selector, vec!["<br>".to_string()]);
    let result = extractor.extract(html, TEST_DOMAIN.to_string()).await;

    let expected = [
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ];

    assert_eq!(result, expected.into());
}