// ensure generic type T has AddAssign/Copy capabilities
fn sum<T: std::ops::AddAssign + Copy>(s: &[T]) -> T {
    let mut total = s[0];

    for i in 1..s.len() {
        total += s[i];
    }

    total
}

fn find<T>(s: &[T], f: fn(&T) -> bool) -> Option<&T> {
    for x in s {
        if f(x) {
            return Some(x);
        }
    }
    None
}

fn find2<T, F>(s: &[T], f: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    for x in s {
        if f(x) {
            return Some(x);
        }
    }
    None
}

fn main() {
    let v = vec![3, 2, 6, 7, 8];
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

    let divisor = 2; // ERROR: can't capture divisor
                     /*
                     fn divisible(x: &i32) -> bool {
                         x % divisor == 0
                     }
                     */

    // closure; can capture environment
    let divisible = |x: &i32| -> bool { x % divisor == 0 };

    if let Some(r) = find2(&v, divisible) {
        println!("{r}");
    }
}
