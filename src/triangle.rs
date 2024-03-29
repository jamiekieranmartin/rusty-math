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
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

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
fn test_triangle_new() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(4.0, 4.0);
    let c = Point::new(4.0, 0.0);

    let triangle = Triangle::new(a, b, c);

    assert!(triangle.a.eq(&a));
    assert!(triangle.b.eq(&b));
    assert!(triangle.c.eq(&c));
}

#[test]
fn test_triangle_area() {
    let a = Point::new(-2.0, 3.0);
    let b = Point::new(-3.0, -1.0);
    let c = Point::new(3.0, -2.0);

    let triangle = Triangle::new(a, b, c);

    let area = triangle.area();

    assert_eq!(area, 12.5);
}

#[test]
fn test_triangle_perimeter() {
    let a = Point::new(-2.0, 3.0);
    let b = Point::new(-3.0, -1.0);
    let c = Point::new(3.0, -2.0);

    let triangle = Triangle::new(a, b, c);

    let perimeter = triangle.perimeter();

    assert_eq!(perimeter, 17.276935967781355);
}
