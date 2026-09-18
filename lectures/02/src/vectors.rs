pub fn run() {
    println!("VECTORS");
    let v = vec![3, 2, 7, 6, 8]; // macro for creating vector with known type
    println!("{v:?}");

    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    // x has typ &i32
    // for x in &v {
    //     println!("{x}");
    // }

    // x hsa type i32
    // for x in v {         // this moves v
    //     println!("{x}");
    // }

    // println!("{v:?}"); // ERROR v is already moved

    let mut v = Vec::new();
    for i in 0..10 {
        v.push(i);
    }

    double(&mut v);
    println!("{v:?}");

    triple(&mut v); // you can pass vectors as slices also
    println!("{v:?}");

    triple(&mut v[1..4]); // can create sub slices
    println!("{v:?}");
}

fn double(v: &mut Vec<i32>) {
    for x in v {
        *x *= 2
    }
}

fn triple(s: &mut [i32]) {
    for x in s {
        *x *= 3
    }
}
