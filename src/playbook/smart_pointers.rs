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

        if (percentage_of_max >= 1.0) {
            self.messenger.send(">=1");
        } else if (percentage_of_max >= 0.9) {
            self.messenger.send(">=0.9");
        } else if (percentage_of_max >= 0.75) {
            self.messenger.send(">=0.75");
        }
    }
}

#[cfg(test)]
mod raw_pointer_tests {

    #[test]
    fn test_pointer_1() {}
}
