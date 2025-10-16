/// CONCEPT: REFERENCE COUNTING
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    rc_example();
    arc_ex();
}

fn rc_example() {
    let s = Rc::new(String::from("Hello Rc"));
    println!("Count after creating s: {}", Rc::strong_count(&s));

    let s1 = Rc::clone(&s);
    println!("Count after cloning s1: {}", Rc::strong_count(&s));

    {
        let s2 = Rc::clone(&s);
        println!("Count after cloning s2: {}", Rc::strong_count(&s));
    } // s2 is out of scope and dropped from the block

    println!("Count after dropping s2: {}", Rc::strong_count(&s));

    // All three point to the same Rust string on the heap. Rust keeps ref count.
} // Count is decreased when Rc goes out of scope. Rust frees heap when Rc is 0.

fn arc_ex() {
    let num = Arc::new(5);
    let mut handles = vec![];

    for _ in 0..3 {
        let num_clone = Arc::clone(&num);
        let handle = thread::spawn(move || { // launch parallel workers
            println!("Value: {}", num_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", Arc::strong_count(&num));
}
