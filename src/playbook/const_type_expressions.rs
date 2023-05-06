// https://practice.rs/generics-traits/const-generics.html

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


pub struct MinSlice<T, const N: usize> {
    /// The bounded region of memory. Exactly `N` `T`s.
    pub head: [T; N],
    /// Zero or more remaining `T`s after the `N` in the bounded region.
    pub tail: [T],
}

fn main() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    // We know this value is `Some(b' ')`,
    // but the compiler can't know that.
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello, world";
    // Length check is performed when we construct a MinSlice,
    // and it's known at compile time to be of length 12.
    // If the `unwrap()` succeeds, no more checks are needed
    // throughout the `MinSlice`'s lifetime.
    // let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    // let value: u8 = minslice.head[6];
    // assert_eq!(value, b' ')
}
