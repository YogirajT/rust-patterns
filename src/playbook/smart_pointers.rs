#![allow(dead_code)]
use std::{cell::RefCell, rc::{Rc, Weak}};
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

struct P {
    i: Rc<RefCell<I>>,
}

impl P {
    fn new() -> P {
        let b = B {
            v: "b".to_string(),
            i: Weak::new(),
        };
        let c = C {
            v: "c".to_string(),
            i: Weak::new(),
        };
        let i = Rc::new(RefCell::new(I { b, c }));
        let ii = i.clone();
        let p = P { i };

        // init b.i
        let mut borrow_mut = RefCell::borrow_mut(&ii);
        let bb = &mut borrow_mut.b;
        bb.i = Rc::downgrade(&ii);

        // init c.i
        let cc = &mut borrow_mut.c;
        cc.i = Rc::downgrade(&ii);
        p
    }

    fn update_bv_cv(&self) {
        let mut borrow_mut = RefCell::borrow_mut(&self.i);

        // update c.v directly
        borrow_mut.c.v.push_str("=>p.update_bv_cv");

        unsafe { (*borrow_mut.b.i.upgrade().unwrap().as_ptr()).c.v.push_str(" b.update_cv"); }
    }

    fn get_bv_cv(&self) -> (String, String) {
        let i = &self.i;
        let ii = i.borrow();

        let bv = ii.b.v.as_str();
        let cv = ii.c.v.as_str();

        (bv.into(), cv.into())
    }
}

// parent inner
struct I {
    c: C,
    b: B,
}

// child
struct C {
    i: Weak<RefCell<I>>,
    v: String,
}

// child
struct B {
    i: Weak<RefCell<I>>,
    v: String,
}


#[cfg(test)]
mod smart_pointer_tests {
    use std::cell::{RefCell, Cell};

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

            // second borrow not allowed as Refcell only allows 1 mutable borrow.
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

    #[test]
    fn refcell_already_borrowed_test() {
        let p = P::new();
        p.update_bv_cv();
        let (bv, cv) = p.get_bv_cv();
        // assert_eq!(bv, "b=>p.update_bv_cv");
        assert_eq!(cv, "c=>p.update_bv_cv b.update_cv");
    }

    #[test]
    fn cell_test() {

        let val = 42;

        let mut cell_store = Cell::new(val);

        assert_eq!(val, cell_store.get());

        let updated_val = 100;

        let previous_val = cell_store.replace(updated_val);

        assert_eq!(updated_val, cell_store.get());

        assert_eq!(previous_val, val);

        cell_store.set(previous_val);

        assert_eq!(val, cell_store.get());

        let cell_ptr = cell_store.as_ptr();

        let unsafe_val = 6;

        unsafe {
            *cell_ptr = unsafe_val;
        }

        assert_eq!(unsafe_val, cell_store.get());

        let mutable_cell = cell_store.get_mut();

        let final_value = 7;

        *mutable_cell = final_value;

        assert_eq!(final_value, cell_store.get());
    }

}
