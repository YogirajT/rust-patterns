#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn add(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> i32 {
        self.count
    }
}

lazy_static! {
    static ref COUNTER_SINGLETON: Arc<Mutex<Counter>> = Arc::new(Mutex::new(Counter::new()));
}

#[cfg(test)]
mod singleton_tests {
    use crate::patterns::creational::singleton::COUNTER_SINGLETON;

    #[test]
    fn test_singleton() {
        let available_threads = 4;

        let mut threads = Vec::new();

        for _ in 0..available_threads {
            let counter_clone = COUNTER_SINGLETON.clone();

            threads.push(std::thread::spawn(move || {
                let mut counter = counter_clone.lock().unwrap();
                counter.add();
            }))
        }

        for th in threads {
            th.join().unwrap();
        }

        let counter_clone = COUNTER_SINGLETON.clone();

        let counter = counter_clone.lock().unwrap();

        assert_eq!(available_threads, counter.get_count());
    }
}
