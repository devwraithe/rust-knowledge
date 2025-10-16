## CONTEXT & EXPECT

### 1. Concept

* Small but powerful tools in Rust’s error-handling system.  
* Mostly used with `Result<T, E>`.  
* Can replace full `match` statements for handling errors quickly.  
* `expect()` **crashes** the program with a custom message — useful when it can’t safely continue (e.g., missing config file).  
* `.context()` adds **rich error explanations** without panicking. Comes from the **anyhow** crate.  
* `.context()` provides a **stacked error message** — it preserves the original cause while adding a higher-level description.  
* `.expect()` → for **non-recoverable** situations.  
  `.context()` → for **recoverable** situations (errors you want to propagate).

---

### 2. How It Works

* `expect()` unwraps a `Result`.  
  * If it’s `Ok`, returns the value.  
  * If it’s `Err`, **panics** and prints your custom message + the system error.  
* `.context()` attaches an explanation to the existing error and passes it up.  
  * It wraps the original error instead of destroying it.  
  * Returns a new `Result<T, anyhow::Error>`.  
  * Makes debugging easier because you see both the high-level context and the root cause.

---

### 3. Example

```rust
/* .expect() */
use std::fs;

let contents = fs::read_to_string("hello.txt")
    .expect("Failed to read file");

// thread 'main' panicked at 'Failed to read file: No such file or directory (os error 2)'

/* .context() */
use anyhow::{Context, Result};
...
    let contents = fs::read_to_string("hello.txt")
        .context("Failed to read file")?;

    println!("{}", contents);
    Ok(())
...

// Error: Failed to read file
//
// Caused by:
//     No such file or directory (os error 2)