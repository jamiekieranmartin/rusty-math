#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

mod area;
mod circle;
mod point;
mod rectangle;
mod triangle;

use crate::area::Area;
use crate::circle::{calc_circle_area, Circle};
use crate::rectangle::{calc_rectangle_area, Rectangle};
use crate::triangle::{calc_triangle_area, Triangle};

#[post("/rectangle/area", data = "<rectangle>", format = "json")]
fn rectangle_area(rectangle: Json<Rectangle>) -> Json<Area> {
    Json(Area {
        area: calc_rectangle_area(&rectangle.into_inner()),
    })
}

#[post("/triangle/area", data = "<triangle>", format = "json")]
fn triangle_area(triangle: Json<Triangle>) -> Json<Area> {
    Json(Area {
        area: calc_triangle_area(&triangle.into_inner()),
    })
}

#[post("/circle/area", data = "<circle>", format = "json")]
fn circle_area(circle: Json<Circle>) -> Json<Area> {
    Json(Area {
        area: calc_circle_area(&circle.into_inner()),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![circle_area, triangle_area, rectangle_area])
}
