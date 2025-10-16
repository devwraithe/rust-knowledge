/// CONCEPT: OWNERSHIP

fn main() {
    println!("Hello, world!");

    basic();
    borrowing();
    mut_borrow();
    lifetime();
}

fn basic() {
    let s1 = String::from("Hello");
    let s2 = s1; // move occurs, s1 invalid from here

    // println!("{}", s1); ❌ invalid — s1 no longer owns the data
    println!("{}", s2); // ✅ s2 is the new owner
}

fn borrowing() {
    let s1 = String::from("Rust");
    print_length(&s1);
    println!("s1 is still valid: {}", s1);
    let len = calculate_length(&s1); // &s1 = borrow

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn mut_borrow() {
    let mut s = String::from("Hello");
    modify(&mut s);
    println!("{}", s);
}

fn lifetime() {
    let r;
    {
        let x = String::from("temp");
        r = &x; // ❌ x will be dropped too soon
    }
    // println!("{}", r); // ❌ dangling reference
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn modify(s: &mut String) {
    s.push_str(", world!");
}
