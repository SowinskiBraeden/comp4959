fn test1() {
    let x = Some(32);
    println!("{}", x.unwrap());
    // let x: Option<i32> = None;
    // println!("{}", x.unrwap()); // panics

    let n = "123".parse::<i32>();
    println!("{n:?}")
}

fn sqrt(x: f64) -> Option<f64> {
    if (x < 0.0) { None } else { Some(f64::sqrt(x)) }
}

fn ln_sqrt(x: f64) -> Option<f64> {
    /*
    match sqrt(x) {
        Some(x) => f64::ln(x),
        None => None,
    }

    // OR
    if let Some(x) = sqrt(x) {
        Some(f64::ln(x))
    } else {
        None
    }
     */

    Some(f64::ln(sqrt(x)?))
}

fn main() {
    test1();
}
