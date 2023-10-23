use rocket::serde::{Deserialize, Serialize};

use crate::point::Point;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn calc_rectangle_area(
    Rectangle {
        top_left,
        bottom_right,
    }: &Rectangle,
) -> f32 {
    let Point { x: x1, y: y1 } = top_left;
    let Point { x: x2, y: y2 } = bottom_right;
    (x2 - x1) * (y2 - y1)
}

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 4.0, y: 4.0 },
    };

    let area = calc_rectangle_area(&rectangle);

    assert_eq!(area, 16.0);
}
