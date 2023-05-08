// https://practice.rs/generics-traits/const-generics.html
#![allow(dead_code)]

use std::slice;

fn foo<const N: usize>() {}


fn bar<T, const M: usize>() {
    foo::<M>();
    foo::<2021>();
    foo::<{20 * 100}>();

    // foo::<{ M + 1 }>(); // Error: const expression contains the generic parameter `M`
    // foo::<{ std::mem::size_of::<T>() }>(); // Error: const expression contains the generic parameter `T`

    let _: [u8; M];
    //let _: [u8; std::mem::size_of::<T>()]; // Error: const expression contains the generic parameter `T`
}


pub struct MinSlice<T: Sized, const N: usize> {
    /// The bounded region of memory. Exactly `N` `T`s.
    pub head: [T; N],
    /// Zero or more remaining `T`s after the `N` in the bounded region.
    pub tail: [T],
}

impl<T: Copy, const N: usize> MinSlice<T, N> {
    pub fn from_slice(slice: &[T]) -> Option<&MinSlice<T, N>> {
        if slice.len() >= N {
            Some(unsafe { Self::from_slice_unchecked(slice) })
        } else {
            None
        }
    }

    pub unsafe fn from_slice_unchecked(slice: &[T]) -> &MinSlice<T, N> {
        let resized = slice::from_raw_parts(slice.as_ptr(), slice.len() - N);
        &*(resized as *const [T] as *const MinSlice<T, N>)
    }
}

#[cfg(test)]
mod const_type_expression_tests {
    use crate::playbook::const_type_expressions::MinSlice;


    #[test]
    fn test_const_type_expression_() {
        let slice: &[u8] = b"Hello, world";
        let reference: Option<&u8> = slice.get(6);
        assert!(reference.is_some());

        let slice: &[u8] = b"Hello, world";

        let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
        let value: u8 = minslice.head[6];
        assert_eq!(value, b' ')
    }

}
