#![allow(dead_code)]
use num::Num;

struct Context<T> {
    strategy: Box<dyn Filter<T>>,
}

struct EvenFilter {}

struct OddFilter {}

impl<T> Context<T> {
    fn new(strategy: Box<dyn Filter<T>>) -> Self {
        Self { strategy }
    }

    fn filter(&self, input: Vec<T>) -> Vec<T> {
        self.strategy.filter(input)
    }
}
trait Filter<T> {
    fn filter(&self, input: Vec<T>) -> Vec<T>;
}

impl<T: Num + Copy> Filter<T> for EvenFilter
where
    <T as num::Num>::FromStrRadixErr: std::fmt::Debug,
{
    fn filter(&self, input: Vec<T>) -> Vec<T> {
        input
            .into_iter()
            .filter(|&x| x.rem(Num::from_str_radix("2", 10).unwrap()) == T::zero())
            .collect()
    }
}

impl<T: Num + Copy> Filter<T> for OddFilter
where
    <T as num::Num>::FromStrRadixErr: std::fmt::Debug,
{
    fn filter(&self, input: Vec<T>) -> Vec<T> {
        input
            .into_iter()
            .filter(|&x| x.rem(Num::from_str_radix("2", 10).unwrap()) == T::one())
            .collect()
    }
}

#[cfg(test)]
mod strategy_tests {
    use crate::patterns::strategy::{Context, EvenFilter, OddFilter};

    #[test]
    fn test_strategy() {
        let ctx_even = Context::new(Box::new(EvenFilter {}));

        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let output_event = ctx_even.filter(input.clone());

        output_event.iter().for_each(|x| assert!(x % 2 == 0));

        let ctx_odd = Context::new(Box::new(OddFilter {}));

        let output_odd = ctx_odd.filter(input);

        output_odd.iter().for_each(|x| assert!(x % 2 == 1));
    }
}
