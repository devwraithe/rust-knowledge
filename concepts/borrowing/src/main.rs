/// CONCEPT: BORROWING

fn main() {
    scope_life();
    basics();
    immutable_borrow();
    mutable_borrow();
    // conflicting_borrows();
}

fn basics() {
    let s1 = String::from("Rust"); // owner
    let len = calculate_length(&s1); // borrow s1
    println!("The length of '{}' is {:?}.", s1, len);
}

fn immutable_borrow() {
    let s = String::from("Hello"); //owner
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2); // both ok, read-only
}

fn mutable_borrow() {
    let mut s = String::from("Hi"); //owner
    change(&mut s);
    println!("{}", s);
}

// fn conflicting_borrows() {
//     let mut s = String::from("Rust");
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s; // cannot borrow as mutable, immutable refs exist
//     println!("{}, {}, {}", r1, r2, r3);
// }

fn scope_life() {
    let mut s = String::from("Rust");
    let r1 = &s; // borrowed as immutable
                 // r1.push_str(" Learn"); // won't work
    println!("R1: {}", r1); // borrow ends
    let r2 = &mut s; // now allowed
    r2.push_str(" Lang");
    println!("R2: {}", r2); // borrow ends
}

fn calculate_length(s: &String) {
    // Read but don't own
    s.len();
}

fn change(s: &mut String) {
    s.push_str(", Borrowing Rust");
}
