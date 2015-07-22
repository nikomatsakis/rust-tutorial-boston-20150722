#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Circle { origin: Point, radius: f32 },
    Rectangle { ul: Point, lr: Point },
}

const PI: f32 = 3.14159;

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}

impl Shape {
    fn unit_circle() -> Shape {
        Shape::circle(0.0, 0.0, 1.0)
    }

    fn circle(x: f32, y: f32, radius: f32) -> Shape {
        Shape::Circle { origin: Point { x: x, y: y }, radius: radius }
    }

    fn rect(ul: Point, lr: Point) -> Shape {
        Shape::Rectangle { ul: ul, lr: lr }
    }

    fn area(&self) -> f32 {
        match *self {
            Shape::Circle { origin: _, radius: r } =>
                PI * r * r,

            Shape::Rectangle { ul, lr } =>
                (lr.y - ul.y).abs() * (lr.x - ul.x).abs()
        }
    }

    /// Returns two points (upper-left, lower-right) that form
    /// a rectangle which completely encloses this shape.
    fn bounding_rectangle(&self) -> (Point, Point) {
        // Exercise 1: implement this function to compute the bounding
        // rectangle for a given shape.
        panic!("Not yet implemented");
    }
}

/// Finds a rectangle bounding all the given shapes.
fn bound_all(shapes: &[Shape]) -> (Point, Point) {
    // Exercise 2: implement this function
    panic!("Not yet implemented");
}

#[test]
fn unit_test1() {
    let c1 = Shape::circle(0.0, 0.0, 1.0);
    let br = (Point::new(-1.0, 1.0), Point::new(1.0, -1.0));
    assert_eq!(c1.bounding_rectangle(), br);
}
