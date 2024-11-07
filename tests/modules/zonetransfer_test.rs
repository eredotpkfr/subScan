use crate::common::{
    constants::{LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::dns::MockDNSServer,
};
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use subscan::{enums::dispatchers::SubscanModuleDispatcher, modules::zonetransfer::ZoneTransfer};
use tokio::sync::Notify;

#[tokio::test]
async fn get_ns_as_ip_test() {
    let notify_one = Arc::new(Notify::new());
    let notift_two = notify_one.clone();

    let zonetransfer = ZoneTransfer::dispatcher();
    let server = MockDNSServer::new(TEST_DOMAIN);
    let addr = server.socket_addr().await;

    tokio::spawn(async move {
        notift_two.notify_one();
        server.start().await;
    });

    notify_one.notified().await;

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        assert_eq!(
            zonetransfer.get_ns_as_ip(addr, TEST_DOMAIN).await.unwrap(),
            [addr]
        );
    }
}

#[tokio::test]
async fn attempt_zone_transfer_test() {
    let notify_one = Arc::new(Notify::new());
    let notift_two = notify_one.clone();

    let zonetransfer = ZoneTransfer::dispatcher();
    let server = MockDNSServer::new(TEST_DOMAIN);
    let addr = server.socket_addr().await;

    tokio::spawn(async move {
        notift_two.notify_one();
        server.start().await;
    });

    notify_one.notified().await;

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        assert_eq!(
            zonetransfer.attempt_zone_transfer(addr, TEST_DOMAIN).await,
            vec![TEST_BAR_SUBDOMAIN]
        );
    }
}

#[tokio::test]
async fn get_async_client_test() {
    let notify_one = Arc::new(Notify::new());
    let notift_two = notify_one.clone();

    let zonetransfer = ZoneTransfer::dispatcher();
    let server = MockDNSServer::new(TEST_DOMAIN);
    let addr = server.socket_addr().await;

    tokio::spawn(async move {
        notift_two.notify_one();
        server.start().await;
    });

    notify_one.notified().await;

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        assert!(zonetransfer.get_async_client(addr).await.is_some());
    }
}

#[tokio::test]
async fn get_async_client_fail_test() {
    let zonetransfer = ZoneTransfer::dispatcher();
    let addr = SocketAddr::from_str(&format!("{LOCAL_HOST}:0")).unwrap();

    if let SubscanModuleDispatcher::ZoneTransfer(zonetransfer) = zonetransfer {
        assert!(zonetransfer.get_async_client(addr).await.is_none());
    }
}
