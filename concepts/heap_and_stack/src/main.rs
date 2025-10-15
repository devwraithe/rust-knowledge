/// Concept: Heap and Stack

fn main() {
    /* --- Stack --- */
    let a = 10; // fixed-size int (4 bytes) lives on stack
    let b = 20; // also lives on stack
    let sum = a + b; // result is an int, also lives on stack

    println!("Sum (Stack): {}", sum);

    /* --- Heap --- */
    let s1 = String::from("Hello"); // heap allocated 5 bytes. stores len, cap, ptr on stack.
    let s2 = s1; // ownership moved, s1 invalid

    // println!("s1: {}", s1); // error: s1 invalid (moved)
    println!("s2: {}", s2); // works. s2 owns heap memory now

    /* --- Stack + Heap --- */
    let mut s3 = String::from("Hi!"); // heap allocated. stack holds ptr, len, cap. mutable.
    println!("s3 (before): {}", s3);

    s3.push_str("I'm Rust"); // grows string. exceed heap cap. trigger heap realloc.
    println!("s3 (after): {}", s3);

    /* --- Stack Frame --- */
    stack_frame(); // creates stack frame for this fn
}

fn stack_frame() {
    let x = 42; // lives in this fn, no where else
    let y = String::from("Stack frame"); // heap data owned by this frame

    println!("In stack_frame() fn: {}, {}", x, y);
} // y dropped -> heap mem free; x popped off stack automatically.