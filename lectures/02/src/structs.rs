use std::fmt;

#[derive(Debug, Copy, Clone)] // derive the implementatoin of the Debug trait (like an interface)
struct Point(f64, f64); // tuple struct

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Circle {
    // or you can specify -> Self
    pub fn new(center: Point, radius: f64) -> Circle {
        // Or you can specify self instead of Circle {
        Circle { center, radius } // since names are the same, we dont need key: value pair
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle: {} {}", self.center, self.radius)
    }
}

pub fn run() {
    println!("STRUCTS");

    let p = Point(1.0, 2.0);
    println!("{} {}", p.0, p.1);

    let Point(x, y) = p;
    println!("{x} {y}");

    let c = Circle {
        center: p,
        radius: 1.0,
    };
    println!("{c:?}");

    let mut c = Circle::new(Point(0.0, 0.0), 1.0);

    println!("{}", c.area()); // area will automatically take reference
    println!("{}", (&c).area()); // or you can specify manually
    c.scale(2.0);
    println!("{c:?}");
    println!("{}", c.area());

    // after implementing Display trait
    println!("{c}");
    let _s = format!("{c}"); // format uses Display trait, to convert to string
}
