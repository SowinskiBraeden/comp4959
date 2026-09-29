// motivation for closures
fn sum<T: std::ops::AddAssign + Copy>(s: &[T]) -> T {
    let mut total = s[0];

    for i in 1..s.len() {
        total += s[i];
    }

    total
}

fn find<T>(s: &[T], f: fn(&T) -> bool) -> Option<&T> {  // only works for functions f
    for x in s {
        if f(x) {
            return Some(x);
        }
    }
    None
}

fn find2<T, F>(s: &[T], f: F) -> Option<&T>  // works for both functions & closures
where F: Fn(&T) -> bool {
    for x in s {
        if f(x) {
            return Some(x);
        }
    }
    None
}

fn main() {
    let v = vec![3, 2, 7, 6, 8];
    println!("{}", sum(&v));

    let v = vec![3.1, 2.0, 7.3, 6.1, 8.0];
    println!("{}", sum(&v));

    let v = vec![3, 2, 7, 6, 8];
    fn is_even(x: &i32) -> bool {
        x % 2 == 0
    }

    if let Some(r) = find(&v, is_even) {
        println!("{r}");
    }

    let divisor = 2;
    /* 
    fn divisible(x: &i32) -> bool {
        x % divisor == 0    // ERROR: can't capture divisor
    }
    */

    // let divisible = |x: &i32| -> bool { x % divisor == 0 };  // closure; can capture environment
    let divisible = |x: &i32| { x % divisor == 0 };  // closure; can capture environment

    /*
    if let Some(r) = find(&v, divisible) {  // ERROR: A closure is not regular fn
        println!("{r}");
    }
    */

    if let Some(r) = find2(&v, divisible) {  // OK
        println!("{r}");
    }
/*
    if let Some(r) = find2(&v, is_even) {  // OK, also works for regular function
        println!("{r}");
    }
*/
}
