#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

mod area;
mod circle;
mod point;
mod rectangle;
mod triangle;

use crate::area::Area;
use crate::circle::Circle;
use crate::rectangle::Rectangle;
use crate::triangle::Triangle;

#[post("/rectangle/area", data = "<rectangle>", format = "json")]
fn rectangle_area(rectangle: Json<Rectangle>) -> Json<Area> {
    Json(Area {
        area: rectangle.area(),
    })
}

#[post("/rectangle/perimeter", data = "<rectangle>", format = "json")]
fn rectangle_perimeter(rectangle: Json<Rectangle>) -> Json<Area> {
    Json(Area {
        area: rectangle.perimeter(),
    })
}

#[post("/triangle/area", data = "<triangle>", format = "json")]
fn triangle_area(triangle: Json<Triangle>) -> Json<Area> {
    Json(Area {
        area: triangle.area(),
    })
}

#[post("/triangle/perimeter", data = "<triangle>", format = "json")]
fn triangle_perimeter(triangle: Json<Triangle>) -> Json<Area> {
    Json(Area {
        area: triangle.perimeter(),
    })
}

#[post("/circle/area", data = "<circle>", format = "json")]
fn circle_area(circle: Json<Circle>) -> Json<Area> {
    Json(Area {
        area: circle.area(),
    })
}

#[post("/circle/perimeter", data = "<circle>", format = "json")]
fn circle_perimeter(circle: Json<Circle>) -> Json<Area> {
    Json(Area {
        area: circle.perimeter(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            circle_area,
            circle_perimeter,
            triangle_area,
            triangle_perimeter,
            rectangle_area,
            rectangle_perimeter,
        ],
    )
}
