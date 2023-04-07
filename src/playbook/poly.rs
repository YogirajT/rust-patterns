#![allow(dead_code)]
#![allow(unused_imports)]
pub trait Ingredient {
    fn stick_it_in_a_stew(&self);
}

pub struct DynamicPotato {}

impl Ingredient for DynamicPotato {
    fn stick_it_in_a_stew(&self) {
        println!("Added Potato!")
    }
}

pub struct DynamicCarrot {}

impl Ingredient for DynamicCarrot {
    fn stick_it_in_a_stew(&self) {
        println!("Added Carrot!")
    }
}

pub struct DynamicChicken {}

impl Ingredient for DynamicChicken {
    fn stick_it_in_a_stew(&self) {
        println!("Added Chicken!")
    }
}

pub fn add_ingredients_to_stew(ingredient: &dyn Ingredient) {
    ingredient.stick_it_in_a_stew();
}

mod poly_tests {
    use super::{add_ingredients_to_stew, DynamicCarrot, DynamicChicken, DynamicPotato};

    #[test]
    fn dynamic_dispatch_test() {
        add_ingredients_to_stew(&DynamicChicken {});
        add_ingredients_to_stew(&DynamicCarrot {});
        add_ingredients_to_stew(&DynamicPotato {});
    }

    #[test]
    fn static_dispatch_test() {}
}
