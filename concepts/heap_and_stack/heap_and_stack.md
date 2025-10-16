## HEAP & STACK

### 1. Concept

* Part of Rust’s memory model.
* Gives automatic memory safety (no Garbage Collector).
* Heap used when size/lifetime unknown at compile time.
* Data → heap, pointer → stack.
* Stack faster (just moves pointer).
* Each fn call = new stack frame, dropped when fn ends.
* Stack = small, fixed, short-lived.
* Heap = large, flexible, long-lived.
* Used to balance speed, flexibility, safety.
* Stack stores ownership handles.
* Heap stores actual contents.

---

### 2. How It Works

* Ints live fully on stack (fixed size).
* Strings: bytes on heap, metadata (`ptr`, `len`, `cap`) on stack.
* `String::from()` → growable string (`s.push('!')`).
* Stack holds pointer + metadata.
* Rust uses system allocator to request/release heap memory.
* `len` = used bytes, `cap` = reserved bytes, `ptr` = start of heap.
* Rust auto-drops vars out of scope.
* For heap → compiler calls Drop → dealloc → memory returned to OS.
* Flow: alloc → init → use → dealloc.

---

### 3. Example

```rust
let s = String::from("Hello");
```

* Alloc: allocator requests memory.
* Init: “Hello” bytes copied to heap.
* Stack: stores ptr, len, cap.
* Use: ownership manages safety.
* Drop: frees heap, stack frame popped.

**Stack = fast & temporary**
**Heap = flexible & long-lived**
→ Rust uses both for speed + safety.
