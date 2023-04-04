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

    async fn state_machine_test() {
        let state = (true, true, true);

        let _stream = stream::unfold(state, |state| async move {
            match state {
                (true, phase2, phase3) => {
                    // do some stuff for phase 1
                    let item = async { 1 }.await;
                    Some((item, (false, phase2, phase3)))
                }
                (phase1, true, phase3) => {
                    // do some stuff for phase 2
                    let item = async { 2 }.await;
                    Some((item, (false, false, phase3)))
                }
                (phase1, phase2, true) => {
                    // do some stuff for phase 3
                    let item = async { 3 }.await;
                    Some((item, (false, false, false)))
                }
                _ => None,
            }
        });

        // assert_eq!(Some(1), stream.next().await);
        // assert_eq!(Some(2), stream.next().await);
        // assert_eq!(Some(3), stream.next().await);
        // assert_eq!(None, stream.next().await);
    }
}
