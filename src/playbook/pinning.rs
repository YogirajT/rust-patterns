#![allow(dead_code)]
use std::{ptr::NonNull, marker::{PhantomPinned}, pin::Pin};


struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned
}


impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);

        let mut_ref = Pin::as_mut(&mut boxed);

        unsafe {
            let unchecked_mut = Pin::get_unchecked_mut(mut_ref);
            unchecked_mut.slice = slice;
        }
        boxed
    }
}

struct Movable {
    data: String,
    slice: NonNull<String>,
}


impl Movable {
    fn new(data: String) -> Box<Self> {
        let res = Movable {
            data,
            slice: NonNull::dangling(),
        };
        let mut boxed = Box::new(res);

        let slice = NonNull::from(&boxed.data);

        let mut_ref = &mut boxed;

        let movableref = mut_ref.as_mut();

        movableref.slice = slice;

        boxed
    }
}


#[cfg(test)]
mod pin_tests {
    use super::*;

    fn move_ref<T>(arg: T) -> T {
        arg
    }

    #[test]
    fn pinning_test() {
        let test_string = "test".to_string();

        let unmoved = Unmovable::new(test_string);

        let still_unmoved = unmoved;

        assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
    }

    #[test]
    fn pinning_negative_test() {
        let test_string = "test to see if this moves or not".to_string();

        let to_be_moved = Movable::new(test_string);

        let moved = move_ref(to_be_moved);

        assert_eq!(moved.slice, NonNull::from(&moved.data));
    }
}
