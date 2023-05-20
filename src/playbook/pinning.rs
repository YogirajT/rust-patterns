#![allow(dead_code)]
use std::{ptr::NonNull, marker::{PhantomPinned}, pin::Pin, task::{Poll, Context}, fmt::Display};

use futures::Future;


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

#[derive(Debug)]
struct Movable {
    data: String,
    slice: NonNull<String>,
}

impl Movable {
    fn new(data: String) -> Self {
        let res = Movable {
            data,
            slice: NonNull::dangling(),
        };
        let mut boxed = res;

        let slice = NonNull::from(&boxed.data);

        let mut_ref = &mut boxed;

        let movableref = mut_ref;

        movableref.slice = slice;

        boxed
    }
}


struct FutureStruct {}

// Future pinning for struct
impl Future for FutureStruct {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }
}

// Future trait into function
impl FutureStruct {
    fn get_something_async(&self) -> impl Future<Output = String> + '_ {
        async {
            "Hello World".to_string()
        }
    }
}



// Slug with Future and pinning

#[derive(Debug)]
pub struct Webpage {
    title: String,
    slug: *const String,
    _marker: PhantomPinned // adding removing this makes or breakes the test #3
}


impl Webpage {
    pub fn new(title: String) -> Self {
        Self {
            title,
            slug: std::ptr::null(),
            _marker: PhantomPinned
        }
    }

    fn collect_slug(/* &mut self */ self: Pin<&mut Self>) {
        // let ref_slug = &self.title as *const _; // _ is automatically inferred as String
        // self.slug = ref_slug;

        let ref_slug = &self.title as *const String;
        let this = unsafe {
            self.get_unchecked_mut()
        };
        this.slug = ref_slug;
    }

    pub fn get_slug(/* &self */ self: Pin<&Self>) -> String { //replaced with pinned reference to self
        let slugptr = unsafe { &*(self.slug) };

        slugptr.replace(" ", "-").to_lowercase()
    }

    fn get_title(&self) -> &str {
        &self.title
    }
}

impl Future for Webpage {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        println!("Webpage:::poll {}", self);
        Poll::Ready(())
    }
}


impl Display for Webpage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = unsafe { Pin::new_unchecked(self) };
        write!(f, "Webpage:`{}` slug:`{}`", this.get_title(), this.get_slug())?;
        Ok(())
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

        let mut still_unmoved = unmoved;

        assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
    }

    #[test]
    fn pinning_negative_test() {
        let test_string = "test to see if this moves or not".to_string();

        let to_be_moved = Movable::new(test_string);

        let mut moved = to_be_moved;

        println!("{:?}", moved);

        //this will trigger error as data is moved but not the address of self referenceing pointer
        // assert_eq!(moved.slice, NonNull::from(&moved.data));
    }


    #[tokio::test(flavor = "current_thread")]
    async fn pinning_test_3() {
        let mut page1: Webpage = Webpage::new("Page 1".to_string());
        let mut page2 = Webpage::new("Page 2".to_string());
        let mut page1 = unsafe { Pin::new_unchecked(&mut page1) };
        let mut page2= unsafe { Pin::new_unchecked(&mut page2) };

        page1.as_mut().collect_slug();
        page2.as_mut().collect_slug();
        std::mem::swap(&mut page1, &mut page2); // This will break without pin as the struct moved however, not the pointers to the string
        page1.await;
        page2.await;
    }
}
