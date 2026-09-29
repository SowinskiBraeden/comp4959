// how can we store circles & rectangles in a container?
// solution 1: use an enum; static dispatch
#[derive(Clone, Copy)]
struct Point(f64, f64);

// #[derive(Clone, Copy)]
enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point),
}

fn area(s: &Shape) -> f64 {
    match *s {
        Shape::Circle(_p, r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(p1, p2) => ((p1.1 - p2.1) * (p1.0 - p2.0)).abs(),
    }
}

fn total_area(v: &[Shape]) -> f64 {
    let mut total = 0.0;

    for s in v {
        total += area(s);
    }
    total
}

fn main() {
    let v: Vec<Shape> = vec![ 
        Shape::Circle(Point(0.0, 0.0), 1.0),
        Shape::Rectangle(Point(1.0, 1.0), Point(2.0, 3.0)),
    ];
    println!("{}", total_area(&v));
}

