# OWNERSHIP

* Ownership is enforced at **compile time**.
* Guarantees **memory safety without a garbage collector (GC)**.
* Ensures **no memory leaks**, **no dangling pointers**, and **no data races**.
* Fundamental laws:
  1. Each value has a **single owner**.
  2. When the owner goes out of scope, the value is **dropped**.
  3. Only **one owner** exists at a time.
* Passing ownership to another variable **moves** it — it doesn’t copy by default (for heap data).
* Trying to free the same memory twice leads to a **double free**, which Rust prevents at compile time.
* Types like integers are stored directly on the **stack** and implement the **Copy** trait automatically.
* Integers (and similar types) are small, fixed-size values, so they are **cheap and safe to duplicate**.
* **Borrowing** is temporary access without taking ownership — it happens through **references**.
* A reference is denoted with the `&` prefix.
* A **mutable borrow** provides **exclusive write access**.
* A **mutable reference** allows changing borrowed data.
* **Lifetimes** prevent invalid (dangling) references.
* You can have **one mutable reference** or **many immutable references**, but not both at once.
* All borrows must end before the original owner is dropped — this prevents **data races** and **simultaneous modification errors**.
* **Lifetimes** tell the compiler how long references are valid.
* Values like integers live on the **stack** and are automatically freed when their stack frame ends.
* `String` or `Vec<T>` are **heap data** with a **stack struct** pointing to their heap allocation.
* When ownership ends, Rust calls `drop()` to **deallocate heap memory** and pop the stack frame entry.
* One mutable reference = one safe, exclusive way to modify data.
* Multiple mutable references = possible **conflicting writes → compile-time error**.
* No reference can outlive the data it points to.