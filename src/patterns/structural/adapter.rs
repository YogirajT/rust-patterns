#![allow(dead_code)]

static SP_MESSAGE: &str = "Specific request";
pub trait TargetTrait {
    fn approve(&self) -> String;
}

pub trait AdapteeTrait {
    fn approve_driving_license(&self) -> String;
}

pub struct ConcreteAdaptee {}

impl AdapteeTrait for ConcreteAdaptee {
    fn approve_driving_license(&self) -> String {
        String::from(SP_MESSAGE)
    }
}

struct Adapter<'a> {
    adaptee: &'a dyn AdapteeTrait,
}

impl<'a> Adapter<'a> {
    fn new(adaptee: &'a dyn AdapteeTrait) -> Adapter {
        Adapter { adaptee }
    }
}

impl<'a> TargetTrait for Adapter<'a> {
    fn approve(&self) -> String {
        self.adaptee.approve_driving_license()
    }
}

#[cfg(test)]
mod adapter_tests {
    use super::{ConcreteAdaptee, Adapter, SP_MESSAGE, AdapteeTrait, TargetTrait};


    #[test]
    fn test_adapter() {
        let adaptee = ConcreteAdaptee {};

        let adapter = Adapter::new(&adaptee);

        assert_eq!(SP_MESSAGE.to_owned(), adapter.approve());
        assert_eq!(adaptee.approve_driving_license(), adapter.approve());
    }
}
