#![allow(dead_code)]
use std::cmp::max;
use std::collections::HashMap;

pub fn longest_unique_substring(input: &str) -> i32 {
    if input.is_empty() {
        return 0;
    }

    if input.len() == 1 {
        return 1;
    }

    let mut max_length = 0;
    let mut visited_char: HashMap<char, bool> = HashMap::new();

    let (mut left, mut right) = (0, 0);

    while right < input.len() as i32 {
        let right_char = input.chars().nth(right as usize).unwrap();

        if visited_char
                .get(&right_char)
                .is_none() ||
                visited_char
                .get(&right_char)
                .is_some_and(|x| !*x)
        {
            visited_char.insert(right_char, true);
        } else {
            max_length = max(max_length, right - left);

            'inner: while left < right {
                let left_char: char = input.chars().nth(left as usize).unwrap();

                if left_char != right_char {
                    visited_char.insert(left_char, false);
                } else {
                    left += 1;
                    break 'inner;
                }
                left += 1;
            }
        }

        right += 1;
    }

    max(max_length, right - left)
}

#[cfg(test)]
mod windowing_tests {
    use super::*;

    #[test]
    fn test_windowing() {
        let input = "aaabcdaaadde".to_owned();
        let result = longest_unique_substring(&input);

        assert_eq!(4, result)
    }
}
