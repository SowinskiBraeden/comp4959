pub fn find_even(s: &[i32]) -> Option<&i32> {
    for x in s {
        if *x % 2 == 0 {
            return Some(x);
        }
    }
    None
}

// can call function in ancestor even if not public
pub fn sextuple(s: &mut [i32]) {
    super::triple(s);
    super::double(s);
}

pub fn find(s: &[i32], f: fn(&i32) -> bool) -> Option<&i32> {
    for x in s {
        if f(x) {
            return Some(x);
        }
    }
    None
}

pub fn find2<T>(s: &[T], f: fn(&T) -> bool) -> Option<&T> {
    for x in s {
        if f(x) {
            return Some(x);
        }
    }
    None
}
