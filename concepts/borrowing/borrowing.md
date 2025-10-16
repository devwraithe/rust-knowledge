# BORROWING

* Borrowing is how Rust lets you access data you don’t own while still enforcing safety.
* It allows **temporary access** to data without taking ownership.
* Rust enforces two borrowing rules:
  1. **Mutable borrow (`&mut T`)** — only one allowed at a time; it can modify data, so concurrent borrows would cause conflicts.
  2. **Immutable borrow (`&T`)** — multiple allowed; they can only read data, so there’s no risk of conflict.
* You **cannot** have a mutable and immutable borrow of the same data at the same time.
* A borrow ends when it’s **no longer used** (its scope ends or the compiler infers it as unused).
* Borrowing doesn’t copy or clone data — it simply creates a **pointer** to the original owner’s memory, tracked by the **borrow checker**.
* Borrowing has **zero runtime cost** — all safety is enforced at **compile time**.
