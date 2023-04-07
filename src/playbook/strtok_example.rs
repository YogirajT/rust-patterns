#![allow(dead_code)]

// Shedding light on variance and subtyping

// Solution is to introduce another lifetime 'b so that the compiler can choose separate life
// strtok is now generic over 2 lifetimes 1. mutable borrow and 2. the string that is being pointed to and we're returning reference into the string being pointed to
/*
   strtok returns the sufiix string after first instance of delimiter by moving pointer
*/

//pub fn strtok<'a, 'b>(input: &'a mut &'b str, delimiter: char) -> &'b str {
//better way
pub fn strtok<'a>(input: &'_ mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = input.find(delimiter) {
        let prefix = &input[..i];
        let suffix = &input[(i + delimiter.len_utf8())..];
        *input = suffix; //input points to the suffix now. in place replacement
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

    fn check_is_static(_: &'static str) {}

    #[test]
    fn test_strtok() {
        let mut s = "foo bar";

        check_is_static(s); // this coarses the lifetime to static

        // compiler can now choose the lifetime separately than picking one and sticking to it
        //
        // &'a mut &'a str      -vs- &'a      mut &'b str
        // &'static mut &'a str -vs- &'static mut &'b str
        // ^ compile error      -vs- ^ no error
        let foo = strtok(&mut s, ' ');

        assert_eq!("foo", foo);
        // assert_eq!("bar", s);
    }

    #[test]
    fn test_strtok_2() {
        let mut s = "foo bar";
        let x = &mut s; // this compiles as mutable references are covariant over lifetime.
                        //borrow of x stops here
                        //&'s mut -> &'until-x mut
        {
            let foo = strtok(&mut s, ' ');
            assert_eq!("foo", foo);
        }

        assert_eq!(s, "bar");
    }
}
