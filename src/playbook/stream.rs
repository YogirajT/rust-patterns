#[cfg(test)]
mod stream_tests {
    use futures::{stream, StreamExt};

    #[tokio::test]
    async fn basic_stream_test() {
        let stream = stream::iter(vec![17, 19]);
        assert_eq!(vec![17, 19], stream.collect::<Vec<i32>>().await);
    }
}
