#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        match percentage_of_max {
            d if d >= 1.0 => self.messenger.send(">=1"),
            d if d >= 0.9 => self.messenger.send(">=0.9"),
            d if d >= 0.75 => self.messenger.send(">=0.75"),
            _ => (),
        };
    }
}

/*
2 Lists which point to the same data using RC
      ┌─────┐
      │     │
      │  1  ├─────┐
      │     │     │
      └─────┘     │
                  │    ┌─────┐        ┌─────┐        ┌─────┐
                  │    │     │        │     │        │     │
                  ├────►  3  ├────────►  4  ├────────► Nil │
                  │    │     │        │     │        │     │
                  │    └─────┘        └─────┘        └─────┘
      ┌─────┐     │
      │     │     │
      │  2  ├─────┘
      │     │
      └─────┘
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[allow(unused_imports)]
use List::{Cons, Nil};

#[cfg(test)]
mod smart_pointer_tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messges: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messges: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut first_borrow = self.sent_messges.borrow_mut();

            // sencond borrow not allowed as Refcell only allows 1 mutable borrow.
            // let mut second_borrow = self.sent_messges.borrow_mut();
            // error: change this to mutable reference
            first_borrow.push(String::from(msg));
            // second_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn test_mock_messenget() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messges.borrow().len(), 1);
    }

    #[test]
    fn linked_list_test() {
        let value = Rc::new(RefCell::new(1));

        // cloning here as we want both value variable and ll_1 to own the value 1
        // wrapped in Rc pointer so ll_2 and ll_3 can share immutable references to ll_1
        let ll_1 = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let ll_2 = Cons(Rc::new(RefCell::new(2)), Rc::clone(&ll_1));
        let _ll_3 = Cons(Rc::new(RefCell::new(3)), Rc::clone(&ll_1));

        *value.borrow_mut() += 10;

        if let Cons(ref _ll_2_1, ref _ll_2_2) = ll_2 {
            let deref = _ll_2_2.clone();

            if let Cons(ref _ll_2_2_1, ref _ll_2_2_2) = *deref {
                let mutated_value = *_ll_2_2_1.borrow();
                assert_eq!(11, mutated_value);
                return;
            }
        }
        panic!("something wrong with the list")
    }
}
