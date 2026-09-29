// how to put different shapes in a container?
// solution 2: use a trait object; late dispatch

struct Point(f64, f64);
struct Circle(Point, f64);
struct Rectangle(Point, Point);

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.1 * self.1 }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { ((self.1.0 - self.0.0) * (self.1.1 - self.0.1)).abs() }
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    let mut total = 0.0;

    for s in shapes {
        total += s.area();
    }
    total
}

fn main() {
    // let x = Box::new(42);

    let v: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle(Point(0.0, 0.0), 1.0)),
        Box::new(Rectangle(Point(1.0, 1.0), Point(2.0, 3.0))),
    ];

    println!("{}", total_area(&v));
    println!("{}", total_area(&v));
}
