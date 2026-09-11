// numbers an be imported,
// but functions need to be marked public with pub
pub fn run() {
    println!("NUMBERS");

    let x = 123; // numbers default to i32
    println!("{x}");
    // println!("{}", x + 100); // this panics if this line is enabled
    println!("{}", x + 1i8); // x must be an i8

    // variables are immutable by default, use mut to be mutable
    let mut x = 2;
    println!("{x}");
    x = 10;
    println!("{x}");

    // REFERENCES (borrowing)
    let r = &x; // shared reference/immutable reference
    // *r = 11; // not allowed - error
    println!("{r}"); // println!() will always deref no matter how many refs, e.g.
    println!("{}", &&&&&&x);

    let r2 = &x;
    println!("{r} {r2}"); // scope of r & r2 is to here

    let s = &mut x; // mutable reference/exclusive reference
    *s = 11; // allowed because s is mutable/exclusive
    println!("{s}");

    double(s); // or double(&mut x)

    // Ojects at any time can have either:
    // many shared references
    // OR
    // exactly 1 mutable reference
    // e.g. DB can have many readers but only 1 writer
    //      but not at the same time

    // therefore:
    // println!("{r}"); will cause `let s = &mut x;` to error
    // because both r & s are in scope at the same time

    /*
       let mut x = 1;       // can be modified
       let r1 = &mut x;     // can use r1 to modify x
       let mut r2 = &x;     // can modify r2
       let mut r3 = &mut x; // can modify r3 & x
    */

    // add float to int
    // should default to f64 unless inferred as other,
    // e.g. this will default to f32 since we add a f32
    let f = 3.14;
    println!("{}", f + 1 as f32); // cast 1 to f32

    let x = 10;
    println!("{}", x + &1);
}

// we need &mut to borrow original and mutate it
fn double(x: &mut i32) {
    *x *= 2;
}
