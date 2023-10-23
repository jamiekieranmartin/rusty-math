use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[test]
fn test_point_new() {
    let x = 1.0;
    let y = 2.0;

    let point = Point::new(x, y);

    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
}

#[test]
fn test_point_eq() {
    let point1 = Point::new(1.0, 2.0);
    let point2 = Point::new(1.0, 2.0);

    assert!(point1.eq(&point2));
}
