use rocket::serde::{Deserialize, Serialize};

use crate::point::Point;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
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
fn test_rectangle_area() {
    let rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 4.0, y: 4.0 },
    };

    let area = rectangle.area();

    assert_eq!(area, 16.0);
}

#[test]
fn test_rectangle_perimeter() {
    let rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 4.0, y: 4.0 },
    };

    let perimeter = rectangle.perimeter();

    assert_eq!(perimeter, 16.0);
}
