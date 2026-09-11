mod utils;

pub fn run() {
    println!("SLICES");

    let a = [3, 2, 7, 7, 8];
    let s1 = &a[..];
    let s2 = &a[1..3];
    let s3 = &a[2..];

    println!("{s1:?} {s2:?} {s3:?}");

    if let Some(r) = utils::find_even(&a) {
        println!("first even integer: {r}");
    } else {
        println!("no even integer");
    }

    let mut a = [3, 2, 7, 6, 8];
    utils::sextuple(&mut a);
    println!("{a:?}");

    fn divisible5(x: &i32) -> bool {
        *x % 5 == 0
    }

    if let Some(r) = utils::find2(&a, divisible5) {
        println!("found: {r}");
    } else {
        println!("not found");
    }
}

pub fn double(s: &mut [i32]) {
    for x in s {
        *x *= 2;
    }
}

pub fn triple(s: &mut [i32]) {
    for x in s {
        *x *= 3;
    }
}
