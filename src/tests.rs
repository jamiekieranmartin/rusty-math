use crate::{calc_rectangle_area, calc_triangle_area, Area, Point, Rectangle, Triangle};

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

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 4.0, y: 4.0 },
    };

    let area = calc_rectangle_area(&rectangle);

    assert_eq!(area, 16.0);
}
