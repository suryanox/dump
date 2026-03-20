// Operator Overloading in Rust?

use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 5, y: 6 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}
