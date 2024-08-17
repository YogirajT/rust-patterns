// The Visitor pattern is used in software development when you need to add new operations 
// to a group of objects without modifying their classes. It is particularly useful in situations 
// where the structure of an object hierarchy is fixed, but the operations that can be performed 
// on these objects are subject to change.


#![allow(dead_code)]

trait Visitor {
    fn visit_int(&mut self, i: i32);
    fn visit_str(&mut self, s: &str);
}

trait Visitable {
    fn accept(&self, visitor: &mut dyn Visitor);
}

struct IntContainer {
    value: i32,
}

struct StringContainer {
    value: String,
}


impl Visitable for IntContainer {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_int(self.value);
    }
}


impl Visitable for StringContainer {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_str(&self.value);
    }
}

struct ConcreteVisitor {
    output: String,
}

impl Visitor for ConcreteVisitor {
    fn visit_int(&mut self, i: i32) {
        self.output.push_str(&format!("Int: {}\n", i));
    }

    fn visit_str(&mut self, s: &str) {
        self.output.push_str(&format!("Str: {}\n", s));
    }
}


#[cfg(test)]
mod visitor_tests {
    use super::*;

    #[test]
    fn visitor_test(){
        let mut visitor = ConcreteVisitor { output: String::new() };
        let int = IntContainer { value: 42 };
        let string = StringContainer { value: String::from("Hello, World!") };
        let objects: Vec<&dyn Visitable> = vec![&int, &string];

        for object in objects {
            object.accept(&mut visitor);
        }

        assert_eq!(visitor.output, "Int: 42\nStr: Hello, World!\n");
    }
}
