# RUST SMART POINTERS MASTERCLASS

## SECTION 1 — Why Smart Pointers Exist

In low-level languages (C/C++), you manually manage memory.
In high-level languages (Python/Java), the runtime (garbage collector) manages memory.

Rust sits in the middle.
Rust gives:

- zero-cost abstractions
- memory safety without a garbage collector
- low-level control when you need it

But Rust’s ownership rules also introduce tension:

- What if multiple owners want to share data?
- What if you need mutation but have only immutable references?
- What if you want heap allocation?
- What if you want runtime polymorphism?
- What if you need thread-safe data sharing?
- What if you want to track lifetimes with types, not runtime logic?

Smart pointers exist to solve these.

---

## SECTION 2 — Foundational Concepts
2.1 What is a Smart Pointer?

A smart pointer is a type that:

- wraps a value

- controls how the value behaves regarding:

    - ownership

    - lifetime

    - borrowing

    - mutability

    - thread safety

    - memory layout

Smart pointers implement traits like:

- Deref

- DerefMut

- Drop

This allows them to behave like regular references, but with extra capabilities.


## 3.1 Box<T> — Single Owner, Heap Allocation
Purpose

- store data on the heap

- allow recursive types

- enable dynamic dispatch (trait objects)

- single owner, no shareability

| Feature          | Value                 |
| ---------------- | --------------------- |
| Heap allocated?  | ✔                     |
| Multiple owners? | ✘                     |
| Mutable?         | ✔ (if box is mutable) |
| Thread-safe?     | N/A                   |
| Borrowing rules  | same as normal        |

## Choosing the Right Smart Pointer (Decision Matrix)

| Goal                             | Use            |
| -------------------------------- | -------------- |
| Heap allocation                  | Box            |
| Shared ownership (single-thread) | Rc             |
| Shared ownership (multi-thread)  | Arc            |
| Shared + mutable (single-thread) | Rc<RefCell<T>> |
| Shared + mutable (multi-thread)  | Arc<Mutex<T>>  |
| Copy interior mutability         | Cell           |
| Break cycles                     | Weak<T>        |
| Lifetime/type-level behavior     | PhantomData    |

## Performance Considerations

| Pointer     | Cost       | Notes                     |
| ----------- | ---------- | ------------------------- |
| Box         | minimal    | heap allocation cost only |
| Rc          | medium     | atomic-free ref counting  |
| Arc         | high       | atomic ops                |
| RefCell     | low/medium | runtime borrow checks     |
| Mutex       | high       | locks + contention        |
| PhantomData | zero       | compile-time only         |
