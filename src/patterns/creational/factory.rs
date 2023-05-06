#[derive(Debug, PartialEq)]
pub enum Paint {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug, PartialEq)]
pub enum Engine {
    V8,
    V16,
}

#[derive(Debug, PartialEq)]
pub enum Body {
    SPORT,
    HATCHBACK,
    SUV,
}

pub struct Car {
    pub paint: Option<Paint>,
    pub engine: Option<Engine>,
    pub body: Option<Body>,
}

impl Car {
    pub fn new(body: Body) -> Car {
        Car {
            body: Some(body),
            engine: None,
            paint: None,
        }
    }

    pub fn set_paint(&mut self, paint: Paint) {
        self.paint = Some(paint);
    }

    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }
}

pub struct CarFactory {}

impl CarFactory {
    pub fn get_family_car() -> Car {
        Car {
            paint: Some(Paint::BLUE),
            body: Some(Body::SUV),
            engine: Some(Engine::V8),
        }
    }

    pub fn get_sports_car() -> Car {
        Car {
            paint: Some(Paint::RED),
            body: Some(Body::SPORT),
            engine: Some(Engine::V16),
        }
    }

    pub fn get_casual_car() -> Car {
        Car {
            paint: Some(Paint::GREEN),
            body: Some(Body::HATCHBACK),
            engine: Some(Engine::V8),
        }
    }
}

#[cfg(test)]
mod factory_tests {
    use crate::patterns::creational::factory::{Body, Engine, Paint};

    use super::CarFactory;

    #[test]
    fn test_car_factory() {
        let family_car = CarFactory::get_family_car();

        assert_eq!(Paint::BLUE, family_car.paint.unwrap());
        assert_eq!(Engine::V8, family_car.engine.unwrap());
        assert_eq!(Body::SUV, family_car.body.unwrap());
    }
}
