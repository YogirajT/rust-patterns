#![allow(dead_code)]

// Shedding light on variance and subtyping
pub fn strtok<'a>(input: &'a mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = input.find(delimiter) {
        let prefix = &input[..i];
        let suffix = &input[(i + delimiter.len_utf8())..];
        *input = suffix;
        prefix
    } else {
        let prefix = *input;
        *input = "";
        prefix
    }
}

#[cfg(test)]
mod strtok_tests {
    use super::strtok;

    #[test]
    fn test_strtok() {
        let mut s = "foo bar";

        let delimiter: char = ' ';

        let foo = strtok(&mut s, delimiter);

        assert_eq!("foo", foo);
        assert_eq!("bar", s);
    }
}
