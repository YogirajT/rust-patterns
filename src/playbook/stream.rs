#[cfg(test)]
mod stream_tests {
    use futures::{stream, StreamExt};

    #[tokio::test]
    async fn basic_stream_test() {
        let stream = stream::iter(vec![1, 2]);
        assert_eq!(vec![1, 2], stream.collect::<Vec<i32>>().await);
    }

    #[tokio::test]
    async fn mut_curr_stream_test() {
        let mut input = 2;
        let mut pow2 = stream::repeat_with(|| {
            let tmp = input;
            input *= 2;
            tmp
        });

        assert_eq!(Some(2), pow2.next().await);
        assert_eq!(Some(4), pow2.next().await);
        assert_eq!(Some(8), pow2.next().await);
        assert_eq!(Some(16), pow2.next().await);
    }
}
