fn main() {
    let v = vec![3, 2, 7, 6, 8];
    println!("{v:?}");
    let c = || { println!("{v:?}"); };  // Fn
    println!("{v:?}");
    c();
    c();

    let mut v = vec![3, 2, 7, 6, 8];
    println!("{v:?}");
    let mut c = |x| { v.push(x); };  // FnMut
    // println!("{v:?}");
    c(1);
    c(2);
    println!("{v:?}");

    let v = vec![3, 2, 7, 6, 8];
    let c = || { drop(v); };  // FnOnce
    // println!("{v:?}");
    c();
    // c();  // ERROR

    let v = vec![3, 2, 7, 6, 8];
    let c = move || { println!("{v:?}"); };  // v moved in c; c is still Fn
    // println!("{v:?}"); 
    c();
    c();
}

