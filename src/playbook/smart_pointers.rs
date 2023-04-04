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

#[cfg(test)]
mod smart_pointer_tests {
    use super::*;

    struct MockMessenger {
        sent_messges: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messges: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messges.push(String::from(msg))
        }
    }

    #[test]
    fn test_mock_messenget() {}
}