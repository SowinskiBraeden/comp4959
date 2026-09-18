pub fn run() {
    println!("STRINGS");

    let s1 = "hello"; // type &str
    let s2 = String::from("world"); // type String

    print_str(s1);
    // print_string(s1);     // ERROR
    // print_string_ref(s1); // ERROR

    print_str(&s2); // also accepts &String (Deref trait)
    print_string_ref(&s2);
    print_string(s2); // moves s2

    let s = "नमस्ते";
    println!("{}", s.len()); // byte count -> 18
    // println!("{}", s[0]); // can't index string
    println!("{:?}", &s[0..3]); // may panic if not at char boundary
    for c in s.chars() {
        println!("{c}");
    }

    println!("{}", s.chars().count());
    println!("{:?}", s.as_bytes());
}

// which version of string function should be used?
// CONCLUSION: function that takes &str is most flexible
fn print_str(s: &str) {
    println!("{s}");
}

fn print_string(s: String) {
    println!("{s}");
}

fn print_string_ref(s: &String) {
    println!("{s}");
}
