pub fn run() {
    println!("TUPLES");

    let t = (1, 3.14);
    println!("{} {}", t.0, t.1);
    println!("{t:?}"); // debug print

    let (x, y) = t; // pattern matching
    println!("{x} {y}");
    println!("{:?}", swap(&t));
}

fn swap(t: &(i32, f64)) -> (f64, i32) {
    (t.1, t.0)
}
