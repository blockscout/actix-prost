use std::{
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering},
};

static TEST_PORT: AtomicUsize = AtomicUsize::new(20000);

pub fn get_test_port() -> usize {
    TEST_PORT.fetch_add(1, Ordering::SeqCst)
}

pub fn get_test_addr() -> SocketAddr {
    format!("[::]:{}", get_test_port()).parse().unwrap()
}
