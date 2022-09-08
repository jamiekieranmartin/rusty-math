#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Area {
    area: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn calc_rectangle_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    (x2 - x1) * (y2 - y1)
}

#[post("/rectangle/area", data = "<rectangle>", format = "json")]
fn rectangle_area(rectangle: Json<Rectangle>) -> Json<Area> {
    Json(Area {
        area: calc_rectangle_area(&rectangle.into_inner()),
    })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

pub fn calc_triangle_area(triangle: &Triangle) -> f32 {
    let Triangle { a, b, c } = triangle;
    let Point { x: x1, y: y1 } = a;
    let Point { x: x2, y: y2 } = b;
    let Point { x: x3, y: y3 } = c;
    ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
}

#[post("/triangle/area", data = "<triangle>", format = "json")]
fn triangle_area(triangle: Json<Triangle>) -> Json<Area> {
    Json(Area {
        area: calc_triangle_area(&triangle.into_inner()),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![triangle_area, rectangle_area])
}
