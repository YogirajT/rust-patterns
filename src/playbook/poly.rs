#![allow(dead_code)]
#![allow(unused_imports)]
pub trait Ingredient {
    fn stick_it_in_a_stew(&self);
    fn add_to_piza_topping(&self) {}
}

pub struct DynamicdPineapple {}

impl Ingredient for DynamicdPineapple {
    fn stick_it_in_a_stew(&self) {
        println!("Added Pineapple!")
    }
}

pub struct DynamicBellPeppers {}

impl Ingredient for DynamicBellPeppers {
    fn stick_it_in_a_stew(&self) {
        println!("Added BellPeppers!")
    }

    fn add_to_piza_topping(&self) {
        println!("Added BellPepper slices");
    }
}

pub struct DynamicChicken {}

impl Ingredient for DynamicChicken {
    fn stick_it_in_a_stew(&self) {
        println!("Added Chicken!")
    }

    fn add_to_piza_topping(&self) {
        println!("Added Chicken Cubes");
    }
}

pub fn add_ingredients_to_stew(ingredient: &dyn Ingredient) {
    ingredient.stick_it_in_a_stew();
}

pub fn add_ingredients_to_pizza(ingredient: &dyn Ingredient) {
    ingredient.add_to_piza_topping();
}

#[cfg(test)]
mod poly_tests {
    use super::{
        add_ingredients_to_pizza, add_ingredients_to_stew, DynamicBellPeppers, DynamicChicken,
        DynamicdPineapple,
    };

    #[test]
    fn dynamic_dispatch_test() {
        println!("Getting the stew ready");
        add_ingredients_to_stew(&DynamicChicken {});
        add_ingredients_to_stew(&DynamicBellPeppers {});
        add_ingredients_to_stew(&DynamicdPineapple {});
    }

    #[test]
    fn static_dispatch_test() {
        println!("Getting the pizza ready");
        add_ingredients_to_pizza(&DynamicChicken {});
        add_ingredients_to_pizza(&DynamicBellPeppers {});
        add_ingredients_to_pizza(&DynamicdPineapple {});
    }
}
