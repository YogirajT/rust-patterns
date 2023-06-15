use std::mem::take;

pub struct Buffer<T> {
    buf: Vec<T>,
}

impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        let buf = self.buf;
        self.buf = Vec::new();
        buf
    }
}

impl<T> Buffer<T> {
    fn get_and_reset_working(&mut self) -> Vec<T> {
        take(&mut self.buf)
    }
}

mod take_tests {
    use super::*;

    #[test]
    fn take_test() {
        let mut buffer = Buffer { buf: vec![1, 2, 3] };
        assert_eq!(buffer.buf.len(), 3);

        assert_eq!(buffer.get_and_reset_working(), vec![1, 2, 3]);
        assert_eq!(buffer.buf.len(), 0);
    }
}
