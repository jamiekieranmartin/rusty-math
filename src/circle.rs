use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[test]
fn test_circle_area() {
    let circle = Circle { radius: 4.0 };

    let area = circle.area();

    assert_eq!(area, 50.265484);
}

#[test]
fn test_circle_perimeter() {
    let circle = Circle { radius: 4.0 };

    let perimeter = circle.perimeter();

    assert_eq!(perimeter, 25.132742);
}
