#![allow(dead_code)]

use std::marker::PhantomData;

// Vec only drops inner types if they implement drop themselves

// if phantom data contains a T it is considered as droppping a T. This for the compiler to know that the types own the T and can drop it.
struct Deserializer<T> {
    // some fields which don't use T
    t: PhantomData<T>,
}

//this signature doesn't own a T so it can't drop a T
struct Deserializer2<T> {
    // marker of covariance
    t: PhantomData<fn() -> T>,
    // another way
    t2: PhantomData<*const T>,
}

// this is diffcult to use as it  won't be able to shorten lifetimes, can't provide 'static in place of 'a
struct Deserializer3<T> {
    // marker of contravariance
    t: PhantomData<fn(T)>,
}

struct Deserializer4<T> {
    // T here is invariant due to combination of covariance and contravariance
    t: PhantomData<fn(T)>,
    t2: PhantomData<fn() -> T>,

    // another way
    t3: PhantomData<*mut T>,
}
