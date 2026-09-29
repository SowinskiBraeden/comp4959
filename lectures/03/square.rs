// simple example of extension trait
// how to add square method to i32
trait Square {
    fn square(self) -> Self;
}

impl Square for i32 {
    fn square(self) -> Self {
        self * self
    }
}

fn main() {
    println!("{}", 32.square());
}
