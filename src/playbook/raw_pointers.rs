#![allow(dead_code)]
use std::ptr::NonNull;

struct Foo {
    a: *mut u64,
    b: *mut u64,
}

struct FooUsingNonNull {
    a: *mut u64,
    b: NonNull<*mut u64>,
}

#[cfg(test)]
mod raw_pointer_tests {
    use std::{mem, ptr::NonNull};

    use crate::playbook::raw_pointers::{Foo, FooUsingNonNull};

    #[test]
    fn test_pointer_1() {
        // NonNull<T> allows `Option<T>`'s discriminant to be collapsed into the pointer:

        // *mut u64:
        assert_eq!(8, mem::size_of::<*mut u64>());
        //Option<*mut u64>:
        assert_eq!(16, mem::size_of::<Option<*mut u64>>());
        //Option<NonNull<*mut u64>>:
        assert_eq!(8, mem::size_of::<Option<NonNull<*mut u64>>>());

        //it works transitively:

        //Option<Foo>:
        assert_eq!(24, mem::size_of::<Option<Foo>>());

        //Option<FooUsingNonNull>
        assert_eq!(16, mem::size_of::<Option<FooUsingNonNull>>());
    }
}
