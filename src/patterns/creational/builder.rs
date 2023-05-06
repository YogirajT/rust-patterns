#![allow(dead_code)]

use super::factory::{Body, Engine, Paint};

pub struct Car {
    pub paint: Option<Paint>,
    pub engine: Option<Engine>,
    pub body: Option<Body>,
}

impl Car {
    pub fn set_body(&mut self, body: Body) {
        self.body = Some(body);
    }

    pub fn set_paint(&mut self, paint: Paint) {
        self.paint = Some(paint);
    }

    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }
}

struct Carbuilder {
    car: Box<Car>,
}

impl Carbuilder {
    pub fn new() -> Carbuilder {
        Carbuilder {
            car: Box::new(Car {
                paint: None,
                body: None,
                engine: None,
            }),
        }
    }

    pub fn add_engine(mut self, engine: Engine) -> Self {
        self.car.set_engine(engine);
        self
    }

    pub fn add_body(mut self, body: Body) -> Self {
        self.car.set_body(body);
        self
    }

    pub fn add_paint(mut self, paint: Paint) -> Self {
        self.car.set_paint(paint);
        self
    }

    pub fn build_car(self) -> Box<Car> {
        self.car
    }
}

#[cfg(test)]
mod builder_tests {

    use crate::patterns::creational::factory::{Body, Engine, Paint};

    use super::Carbuilder;

    #[test]
    fn test_car_factory() {
        let red_v16_sport_car = Carbuilder::new()
            .add_body(Body::SPORT)
            .add_engine(Engine::V16)
            .add_paint(Paint::RED)
            .build_car();

        assert_eq!(Paint::RED, red_v16_sport_car.paint.unwrap());
        assert_eq!(Engine::V16, red_v16_sport_car.engine.unwrap());
        assert_eq!(Body::SPORT, red_v16_sport_car.body.unwrap());
    }
}
