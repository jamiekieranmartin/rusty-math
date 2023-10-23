use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Circle {
    radius: f32,
}

pub fn calc_circle_area(Circle { radius }: &Circle) -> f32 {
    std::f32::consts::PI * radius * radius
}

#[test]
fn test_circle_area() {
    let circle = Circle { radius: 4.0 };

    let area = calc_circle_area(&circle);

    assert_eq!(area, 50.265484);
}
