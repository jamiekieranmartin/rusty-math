use rocket::serde::{Deserialize, Serialize};

use crate::point::Point;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.a;
        let Point { x: x2, y: y2 } = self.b;
        let Point { x: x3, y: y3 } = self.c;
        ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
    }

    pub fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.a;
        let Point { x: x2, y: y2 } = self.b;
        let Point { x: x3, y: y3 } = self.c;
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
            + ((x2 - x3).powi(2) + (y2 - y3).powi(2)).sqrt()
            + ((x3 - x1).powi(2) + (y3 - y1).powi(2)).sqrt()
    }
}

#[test]
fn test_triangle_area() {
    let triangle = Triangle {
        a: Point { x: -2.0, y: 3.0 },
        b: Point { x: -3.0, y: -1.0 },
        c: Point { x: 3.0, y: -2.0 },
    };

    let area = triangle.area();

    assert_eq!(area, 12.5);
}

#[test]
fn test_triangle_perimeter() {
    let triangle = Triangle {
        a: Point { x: -2.0, y: 3.0 },
        b: Point { x: -3.0, y: -1.0 },
        c: Point { x: 3.0, y: -2.0 },
    };

    let perimeter = triangle.perimeter();

    assert_eq!(perimeter, 17.211102);
}
