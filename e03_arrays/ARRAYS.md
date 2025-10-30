### A fixed-sized array in Rust has:

- A known number of elements at compile time.
- Elements stored contiguously on the stack.
- Type: [T; N] — where T is the type, and N is the constant length.

| Property            | Description                                              |
| ------------------- | -------------------------------------------------------- |
| **Size**            | Known at compile-time                                    |
| **Memory location** | Stack                                                    |
| **Performance**     | Extremely fast (no heap allocation, minimal overhead)    |
| **Mutability**      | Must be declared `mut` to change contents                |
| **Safety**          | Indexing out of bounds causes panic                      |
| **Type signature**  | `[T; N]`, both type and size are part of the type system |

### Why Use Fixed Arrays

1. Performance-critical sections

2. Ideal for embedded systems, numerical computation, or kernel-level logic.

3. Small, constant-size datasets

    - RGB pixel [u8; 3]

    - 3D coordinate [f64; 3]

4. Stack-based deterministic behavior

    - No heap allocation → predictable memory access and cache-friendly.

## VECTORS
A vector is a growable, heap-allocated array.

| Property            | Description                                               |
| ------------------- | --------------------------------------------------------- |
| **Size**            | Determined at runtime                                     |
| **Memory location** | Heap (pointer, capacity, and length on stack)             |
| **Performance**     | Slightly slower — heap allocations, possible reallocation |
| **Mutability**      | Must be declared `mut` to modify contents                 |
| **Safety**          | Indexing out of bounds causes panic                       |
| **Type signature**  | `Vec<T>` — no compile-time length constraint              |


## WHEN TO USE EACH


| Scenario                                          | Recommended |
| ------------------------------------------------- | ----------- |
| You know the size at compile time                 | `[T; N]`    |
| Data size changes dynamically                     | `Vec<T>`    |
| Performance-critical code (tight loops, embedded) | `[T; N]`    |
| Reading data from unknown source (file, API)      | `Vec<T>`    |
| Fixed small configuration like `[u8; 4]` (IPv4)   | `[T; N]`    |
| Growing logs, lists, etc.                         | `Vec<T>`    |


### Arrays and Vectors Must Have a Single Type

By design, Rust arrays `([T; N])` and vectors `(Vec<T>)` must contain elements of the same type.

This rule gives Rust:

- Compile-time memory safety
- Predictable size and layout
- Speed and zero-cost abstractions

```rust
let arr = [1, 2, 3];     // ✅ all i32
let v = vec!["a", "b"];  // ✅ all &str

let bad = [1, "a"];      // ❌ type mismatch
```

### But Sometimes, We Need Dynamic Types

#### You might want to store heterogeneous data — for example:

- A list of numbers, strings, and booleans (like JSON)
- UI components of different structs
- Trait objects (e.g., multiple types implementing a common interface)

To achieve this, Rust provides type-erasure mechanisms like:
```rust
    enum (static sum types)
    trait objects (Box<dyn Trait>)
    Any from std::any
    (and in dynamic contexts) serde_json::Value
```







