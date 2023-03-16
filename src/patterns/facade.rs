#![allow(dead_code)]

struct ComplexWorkflow1 {}

trait WorkflowTrait {
    fn procedure_1(&self) -> String;
    fn procedure_2(&self) -> String;
}

impl WorkflowTrait for ComplexWorkflow1 {
    fn procedure_1(&self) -> String {
        String::from("Procedure 1 called")
    }

    fn procedure_2(&self) -> String {
        String::from("Procedure 2 called")
    }
}

struct ComplexWorkflow2 {}

impl WorkflowTrait for ComplexWorkflow2 {
    fn procedure_1(&self) -> String {
        String::from("Procedure 3 called")
    }

    fn procedure_2(&self) -> String {
        String::from("Procedure 4 called")
    }
}

struct Facade<'a> {
    wf_1: &'a ComplexWorkflow1,
    wf_2: &'a ComplexWorkflow2,
}

impl<'a> Facade<'a> {
    fn new(wf_1: &'a ComplexWorkflow1, wf_2: &'a ComplexWorkflow2) -> Self {
        Self { wf_1, wf_2 }
    }

    fn operation(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.wf_1.procedure_1());
        result.push_str(&self.wf_2.procedure_1());
        result.push_str(&self.wf_1.procedure_2());
        result.push_str(&self.wf_2.procedure_2());

        result
    }
}

#[cfg(test)]
mod facade_tests {
    use super::{ComplexWorkflow1, ComplexWorkflow2, Facade};

    #[test]
    fn test_facade() {
        let facade = Facade::new(&ComplexWorkflow1 {}, &ComplexWorkflow2 {});

        let result = facade.operation();

        assert!(result.contains("Procedure 1"));
        assert!(result.contains("Procedure 2"));
        assert!(result.contains("Procedure 3"));
        assert!(result.contains("Procedure 4"));
    }
}
