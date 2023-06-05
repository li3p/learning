use std::cell::Cell;

struct Point {
    x: Cell<i32>,
    y: Cell<i32>,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }

    fn move_by(&self, dx: i32, dy: i32) {
        self.x.set(self.x.get() + dx);
        self.y.set(self.y.get() + dy);
    }
}

fn main() {
    let p = Point::new(0, 0);
    println!("Point: ({}, {})", p.x.get(), p.y.get());
    p.move_by(10, 20);
    println!("Point: ({}, {})", p.x.get(), p.y.get());
}
