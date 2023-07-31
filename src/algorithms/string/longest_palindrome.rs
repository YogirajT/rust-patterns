#![allow(dead_code)]

fn longest_palindrome_by_expansion(input: &str) -> Option<&str> {
    let input_len = input.len();
    let mut start = 0;
    let mut end = 1;

    for i in 0..input_len {
        let mut low: i32 = i as i32 - 1;
        let mut high: i32 = i as i32;

        while low >= 0
            && high < input_len as i32
            && input.chars().nth(low as usize).unwrap() == input.chars().nth(high as usize).unwrap()
        {
            if high - low + 1 > end {
                start = low;
                end = high - low + 1
            }
            low -= 1;
            high += 1;
        }

        low = i as i32 - 1;
        high = i as i32 + 1;

        while low >= 0
            && high < input_len as i32
            && input.chars().nth(low as usize).unwrap() == input.chars().nth(high as usize).unwrap()
        {
            if high - low + 1 > end {
                start = low;
                end = high - low + 1
            }
            low -= 1;
            high += 1;
        }
    }

    return input.get(start as usize..end as usize - 1);
}

#[cfg(test)]
mod palindrome_tests {
    use super::*;

    #[test]
    fn test_expansion_palindrome() {
        let test_string = "racecar".to_owned();

        let answer = longest_palindrome_by_expansion(&test_string).unwrap();

        assert_eq!("racecar", answer);
    }
}
