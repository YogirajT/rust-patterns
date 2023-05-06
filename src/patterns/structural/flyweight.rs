#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Team {
    Dire,
    Radiant,
}

#[derive(Clone)]
enum Item {
    Club,
    Wand,
}

#[derive(Clone)]
pub struct Character {
    team: Team,
    starting_item: Item,
}

struct Hero {
    character: Character,
    name: &'static str,
    health: i32,
}

pub struct HeroFactory {
    pub shared_data_map: HashMap<String, Character>,
}

impl HeroFactory {
    fn new() -> Self {
        HeroFactory {
            shared_data_map: HashMap::new(),
        }
    }

    fn get_hero(&mut self, team: Team, name: &'static str) -> Hero {
        let shared_data = self
            .shared_data_map
            .entry(format!("{:?}", team))
            .or_insert_with(|| Character {
                team: team.clone(),
                starting_item: if team.eq(&Team::Dire) {
                    Item::Club
                } else {
                    Item::Wand
                },
            });

        Hero {
            character: shared_data.clone(),
            name,
            health: 100,
        }
    }
}

#[cfg(test)]
mod flyweight_tests {
    use super::{HeroFactory, Team};

    #[test]
    fn test_flyweight() {
        let mut hero_factory = HeroFactory::new();

        static INVOKER: &str = "Invoker";
        static BLOOD_SEEKER: &str = "Bloodseeker";
        static BROOD_MOTHER: &str = "Brood Mother";
        static SVEN: &str = "Sven";
        static CM: &str = "Crystal Maiden";
        static IO: &str = "Io";

        let hero1 = hero_factory.get_hero(Team::Dire, INVOKER);
        let hero2 = hero_factory.get_hero(Team::Dire, BLOOD_SEEKER);
        let hero3 = hero_factory.get_hero(Team::Dire, BROOD_MOTHER);
        let hero4 = hero_factory.get_hero(Team::Radiant, SVEN);
        let hero5 = hero_factory.get_hero(Team::Radiant, CM);
        let hero6 = hero_factory.get_hero(Team::Radiant, IO);

        assert_eq!(hero_factory.shared_data_map.len(), 2);

        let dire_team = format!("{:?}", &Team::Dire);
        let radiant_team = format!("{:?}", &Team::Dire);
        assert!(hero_factory.shared_data_map.contains_key(&dire_team));
        assert!(hero_factory.shared_data_map.contains_key(&radiant_team));

        assert_eq!(hero1.name, INVOKER);
        assert_eq!(hero2.name, BLOOD_SEEKER);
        assert_eq!(hero3.name, BROOD_MOTHER);
        assert_eq!(hero4.name, SVEN);
        assert_eq!(hero5.name, CM);
        assert_eq!(hero6.name, IO);
    }
}
