use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[test]
fn test_circle_new() {
    let radius: f64 = 4.0;

    let circle = Circle::new(radius);

    assert_eq!(circle.radius, radius);
}

#[test]
fn test_circle_area() {
    let radius: f64 = 4.0;

    let circle = Circle::new(radius);

    let area = circle.area();

    assert_eq!(area, 50.26548245743669);
}

#[test]
fn test_circle_perimeter() {
    let radius: f64 = 4.0;

    let circle = Circle::new(radius);

    let perimeter = circle.perimeter();

    assert_eq!(perimeter, 25.132741228718345);
}
