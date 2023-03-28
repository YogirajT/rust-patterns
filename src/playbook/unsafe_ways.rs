#![allow(dead_code)]
use std::{panic, slice};

pub fn split_as_mut(vec: &mut [i32], position: usize) -> (&mut [i32], &mut [i32]) {
    let len = vec.len();
    let pointer = vec.as_mut_ptr();

    assert!(position <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(pointer, position),
            slice::from_raw_parts_mut(pointer.add(position), len - position),
        )
    }
}

pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(
    f: F,
) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

#[cfg(test)]
mod unsafe_ways_tests {
    #[cfg(feature = "expensive_tests")]
    use super::catch_unwind_silent;
    use super::split_as_mut;

    #[test]
    fn test_split_as_mut() {
        let mut vec1 = vec![1, 2, 3, 4];

        let (a1, b1) = split_as_mut(&mut vec1, 2);

        assert_eq!(&mut vec![1, 2], a1);
        assert_eq!(&mut vec![3, 4], b1);
    }

    #[test]
    fn test_split_as_mut_0() {
        let mut vec1 = vec![1, 2, 3, 4];

        let (a1, b1) = split_as_mut(&mut vec1, 0);
        assert_eq!(&mut vec![1, 2, 3, 4], b1);
        assert_eq!(&mut Vec::new() as &mut Vec<i32>, a1);
    }

    #[test]
    fn test_split_as_mut_2() {
        let mut vec1 = vec![1, 2, 3, 4];

        let (a1, b1) = split_as_mut(&mut vec1, 4);
        assert_eq!(&mut vec![1, 2, 3, 4], a1);
        assert_eq!(&mut Vec::new() as &mut Vec<i32>, b1);
    }

    #[test]
    #[should_panic]
    fn test_split_as_mut_edge() {
        let mut vec1 = vec![1, 2, 3, 4];

        let (_a1, _b1) = split_as_mut(&mut vec1, 5);
    }

    #[test]
    fn test_split_as_mut_edge2() {
        let mut vec1 = vec![1, 2, 3, 4];

        let result = std::panic::catch_unwind(move || {
            let (_a1, _b1) = split_as_mut(&mut vec1, 5);
        });
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "expensive_tests")]
    fn test_split_as_mut_edge3() {
        let mut vec1 = vec![1, 2, 3, 4];

        let result = catch_unwind_silent(move || {
            let (_a1, _b1) = split_as_mut(&mut vec1, 5);
        });
        assert!(result.is_err());
    }
}
