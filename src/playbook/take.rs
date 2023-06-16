#![allow(dead_code)]
use std::{mem::take};

pub struct Buffer {
    buf: WrappedVec,
}

// this throws an error
// impl<T> Buffer<T> {
//     fn get_and_reset(&mut self) -> Vec<T> {
//         let buf = self.buf;
//         self.buf = Vec::new();
//         buf
//     }
// }

struct WrappedVec {
    vec: Vec<i32>,
}

impl WrappedVec {
    fn new() -> Self {
        WrappedVec { vec: vec![1] }
    }
}

impl Default for WrappedVec {
    fn default() -> Self {
        WrappedVec { vec: vec![1, 2] }
    }
}

impl Buffer{
    fn get_and_reset_working(&mut self) -> WrappedVec{
        take(&mut self.buf)
    }
}

mod take_tests {
    use super::*;

    #[test]
    fn take_test() {
        let mut buffer = Buffer {
            buf: WrappedVec::new(),
        };
        assert_eq!(buffer.buf.vec.len(), 1);

        assert_eq!(buffer.get_and_reset_working().vec, vec![1]);

        // takes value from default impl
        assert_eq!(buffer.buf.vec.len(), 2);
    }
}
