## 5. PROFESSIONAL ENGINEERING HINTS
### 5.1. Design Ownership Boundaries Early

Ask:

- Who owns which data?

- Where should mutation happen?

- Should this function take ownership (T), borrow (&T), or mutate (&mut T)?


### 5.2. Follow the Standard Library Patterns

Use:

- &str instead of String in function parameters

- &[T] instead of Vec<T>

- Cow<T> if sometimes owned, sometimes borrowed

- Arc<T> for shared data in threaded programs


### 5.3. Prefer Immutability

Rust embraces immutability.
You borrow mutably only at the point of change.

### 5.4. Avoid Large Clones

If you find yourself writing `.clone()` everywhere, you likely have an **ownership design flaw**.

### 5.5. Use cargo clippy

Clippy points out unnecessary clones, poor borrow usage, and ownership anti-patterns.

### 5.6. Think of References as "Temporary Visitors"

Allow:

- Many cold visitors (immutable)

- Only one warm visitor (mutable)

- Never allow a visitor to outlive the host (lifetimes)

### 6. ADVANCED PATTERNS TO MASTER NEXT

Here’s what you should study next (I can explain them if you want):

- Rc<T> for shared ownership in single-thread environments

- Arc<T> + Mutex<T> for concurrency

- Interior mutability (Cell, RefCell)

- Zero-cost abstractions with Deref

- Borrow checker errors and how to rewrite code to satisfy them