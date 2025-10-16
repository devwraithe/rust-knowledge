## REFERENCE COUNTING

### 1. Concept

* **RC** = Reference Counting.
* Reference counting consists of `Rc<T>` and `Arc<T>`.
* Rust’s ownership rule: every value has exactly one owner.
* Reference counting allows creation of **multiple owners** of the same heap data.
* It also keeps track of **how many owners** exist at any time.
* `Rc<T>` = **Reference Counter**.
* Part of `std::rc`, used in **single-threaded** programs.
* The control block is freed when all owners are gone, ensuring memory is reclaimed automatically.
* Reference counting alone **cannot detect cyclic references** (e.g., two `Rc`s pointing to each other).
* `Weak<T>` is a **non-owning pointer** that doesn’t increment the strong count.
* `Weak<T>` prevents circular ownership (i.e., cyclic references).
* `Arc<T>` is just like `Rc<T>` but **thread-safe**.
* `Arc<T>` = **Atomic Reference Counter**.
* `Arc<T>` uses **atomic operations** to allow safe shared ownership between multiple threads.
* `Arc<T>` is slower than `Rc<T>` because of its atomicity, but safer for concurrency.
* `Rc<T>` → non-atomic, lightweight, fast, best for single-thread use.
* `Arc<T>` → atomic, slower, best for multi-threaded use.

---

### 2. How It Works

* On `Rc<T>` creation, a small **control block** is allocated on the heap.
* The control block contains:
  - `value` → the actual data.
  - `strong_count` → counts active owners.
  - `weak_count` → counts non-owning references.
* Each `Rc::clone()` increments the `strong_count` (active owners).
* When an `Rc` is dropped, `strong_count` decreases.
* If `strong_count == 0`, the data is dropped.
* If both `strong_count` and `weak_count` == 0, the control block is freed.
* An `Rc` or `Arc` goes out of scope when its **stack frame ends**.
* Each time an `Rc` variable ends, the **reference count decrements**.
* When the count reaches zero, the **heap data is freed**.