#[cfg(test)]
mod async_tests {

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn async_test() {
        assert!(true);
    }

    #[test]
    fn async_test2() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                assert!(true);
            })
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_test3() {
        assert!(true);
    }
}
