pub fn run() {
    println!("ARRAYS");

    // in rust arrays are values
    // unlike in c where arrays are an address
    let a = [3, 2, 7, 6, 8];
    println!("{a:?}"); // debug print

    let mut a = [3, 2, 7, 6, 8];
    a[0] = -3;
    println!("{a:?}");

    print_array5(a);
    double_array5(&mut a);
    print_array5_v2(&a);

    super::slices::double(&mut a);
}

fn print_array5(a: [i32; 5]) {
    // 0..5 is non inclusive
    for i in 0..5 {
        println!("{}", a[i]);
    }

    /* this works also
       x is i32
    for x in a {
        println!("{x}")
    }
    */
}

fn print_array5_v2(a: &[i32; 5]) {
    // x is &i32
    for x in a {
        println!("{x}")
    }
}

fn double_array5(a: &mut [i32; 5]) {
    /*
     * this works
        for i in 0..5 {
            a[i] *= 2;
        }
    */

    for x in a {
        *x *= 2;
    }
}
