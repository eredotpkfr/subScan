use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};
use subscan::extractors::regex::RegexExtractor;
use subscan::interfaces::extractor::SubdomainExtractorInterface;

#[tokio::test]
async fn extract_one_test() {
    let extractor = RegexExtractor::default();

    let matches = String::from(TEST_BAR_SUBDOMAIN);
    let no_match = String::from("foobarbaz");

    assert!(extractor
        .extract_one(matches, TEST_DOMAIN.to_string())
        .is_some());
    assert!(extractor
        .extract_one(no_match, TEST_DOMAIN.to_string())
        .is_none());
}

#[tokio::test]
async fn extract_test() {
    let content = String::from("bar.foo.com\nbaz.foo.com");

    let extractor = RegexExtractor::default();
    let result = extractor.extract(content, TEST_DOMAIN.to_string()).await;

    let expected = [
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ];

    assert_eq!(result, expected.into());
}
