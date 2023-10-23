use rocket::serde::{Deserialize, Serialize};

use crate::point::Point;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;
        (x2 - x1) * (y2 - y1)
    }

    pub fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;
        2.0 * (x2 - x1) + 2.0 * (y2 - y1)
    }
}

#[test]
fn test_rectangle_new() {
    let top_left = Point::new(0.0, 0.0);
    let bottom_right = Point::new(4.0, 4.0);

    let rectangle = Rectangle::new(top_left, bottom_right);

    assert!(rectangle.top_left.eq(&top_left));
    assert!(rectangle.bottom_right.eq(&bottom_right));
}

#[test]
fn test_rectangle_area() {
    let top_left = Point::new(0.0, 0.0);
    let bottom_right = Point::new(4.0, 4.0);

    let rectangle = Rectangle::new(top_left, bottom_right);

    let area = rectangle.area();

    assert_eq!(area, 16.0);
}

#[test]
fn test_rectangle_perimeter() {
    let top_left = Point::new(0.0, 0.0);
    let bottom_right = Point::new(4.0, 4.0);

    let rectangle = Rectangle::new(top_left, bottom_right);

    let perimeter = rectangle.perimeter();

    assert_eq!(perimeter, 16.0);
}
