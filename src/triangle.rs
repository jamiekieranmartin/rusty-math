use rocket::serde::{Deserialize, Serialize};

use crate::point::Point;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

pub fn calc_triangle_area(Triangle { a, b, c }: &Triangle) -> f32 {
    let Point { x: x1, y: y1 } = a;
    let Point { x: x2, y: y2 } = b;
    let Point { x: x3, y: y3 } = c;
    ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
}

#[test]
fn test_triangle_area() {
    let triangle = Triangle {
        a: Point { x: -2.0, y: 3.0 },
        b: Point { x: -3.0, y: -1.0 },
        c: Point { x: 3.0, y: -2.0 },
    };

    let area = calc_triangle_area(&triangle);

    assert_eq!(area, 12.5);
}
