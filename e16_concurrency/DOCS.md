# Rust Concurrency — Full Professional Masterclass

**From first principles → production patterns → when/how to use each tool.**
This guide is designed to be the single reference you return to while building concurrent systems in Rust: threads, ownership rules, message passing, shared state, synchronization primitives, scoped threads, parking, high-level patterns, async vs threads, debugging & profiling, and recommended exercises/projects.

---

## Table of contents (jump to a section)

1. High-level concurrency principles
2. Rust safety primitives: `Send` & `Sync` (ownership model)
3. Threads 101 — `std::thread::spawn`, `join` and examples
4. Scoped threads vs `spawn` — when & how to use them
5. Message passing — `mpsc`, `crossbeam::channel`, patterns (worker pool, fan-out/fan-in)
6. Shared state — `Mutex`, `RwLock`, `Atomic*`, `RefCell/Cell` caveats, `Arc`
7. Condvar, Barrier, Once, and synchronization patterns
8. Parking & thread control — `park()`, `unpark()`, parking_lot crate
9. Thread pools, Rayon, and parallel iterators
10. Async vs threads — differences, when to use each, interoperability
11. Lock-free data structures & atomics — guidelines and pitfalls
12. Actor model, message-driven architectures, futures-based actors
13. Deadlocks, livelocks, poisoning, and avoidance strategies
14. Testing & debugging concurrent code — tools and techniques
15. Performance considerations & microbenchmarking concurrent code
16. Production patterns and architecture examples
17. Exercises / projects (graded roadmap)
18. Further reading & resources

---

## 1 — High-level concurrency principles

* **Concurrency** = structuring a program to do multiple tasks in overlapping time.
* **Parallelism** = doing multiple tasks at the same time (multi-core).
* Rust gives **memory-safety guarantees** at compile-time — *data races* are prevented by the type system (Send/Sync). Synchronization is still your responsibility (correctness & liveness).
* Design goals: minimize locking, prefer message passing where possible, keep shared state small and well-encapsulated, test deterministically where feasible.

---

## 2 — `Send` & `Sync` — Ownership model for threads

* `Send`: a type can be *moved* to another thread. Most types that own data are `Send`. `Rc<T>` is **not** `Send`; `Arc<T>` is `Send` if `T: Send + Sync` as required.
* `Sync`: a type is safe to *share* references between threads (`&T` across threads). If `T: Sync`, then `&T` is `Send`.
* The compiler enforces at compile time; if you hit an error like `type is not Send`, you need to change ownership (Arc) or make the contained types thread-safe.
* `unsafe` can override, but only when you fully understand invariants.

---

## 3 — Threads 101: spawn, join, ownership examples

### Basic spawn & join

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("hello from a thread");
        42
    });

    let result = handle.join().expect("thread panicked");
    println!("thread returned {}", result);
}
```

### Moving ownership into thread

```rust
let data = vec![1,2,3];
let handle = thread::spawn(move || {
    println!("{:?}", data); // data moved into closure
});
```

### Panics

* If a thread panics, `join()` returns `Err` with a `Box<Any + Send>`. You can recover or crash; design policy for panics (restart thread, escalate).
* `Mutex` poisoning: if a thread panics while holding a Mutex, subsequent `lock()` returns a `PoisonError`. You can `unwrap_or_else(|pe| pe.into_inner())` or handle explicitly.

---

## 4 — Scoped threads vs `spawn` — lifetimes & use-cases

### `std::thread::spawn` (detached until joined)

* Good for independent work with `'static` data or moved ownership via `Arc` or `move`.
* Requires moving/owning data, often via `Arc` for shared state.

### `std::thread::scope` (scoped threads)

* Allows threads borrowing data from the parent stack safely; the scope waits until all threads complete.
* Example (stable since Rust 1.63-ish; check your toolchain):

```rust
use std::thread;

fn main() {
    let mut v = vec![1,2,3];
    thread::scope(|s| {
        s.spawn(|| {
            // can't borrow v immutably/mutably here because of closure borrow rules,
            // but you *can* borrow short-lived references safely
        });
    });
    // v is still available here
}
```

* Use scoped threads when you want threads to temporarily borrow stack data without `Arc`, to avoid heap/atomic overhead. Use `spawn` when threads live beyond the current scope or when `Arc` makes sense.

**Rule of thumb**:

* Use `scope()` for short-lived threads with stack borrows (no `Arc`).
* Use `spawn()` + `Arc` for long-lived threads or where `'static` data is OK.

---

## 5 — Message passing: `mpsc` and `crossbeam::channel`

### std::sync::mpsc

* simple, built-in multi-producer single-consumer channel.
* `Sender<T>` is `Clone` for multiple producers.
* Receiver has `recv()` (blocking) and `try_recv()` (non-blocking).

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
for i in 0..5 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}
for _ in 0..5 {
    println!("got {}", rx.recv().unwrap());
}
```

### crossbeam::channel

* high-performance, multi-producer multi-consumer, select support, bounded/unbounded, better performance and features (recommended).
* `select!` macro allows waiting on multiple channels.

```rust
use crossbeam::channel;

let (s, r) = channel::unbounded();
```

### Patterns

* **Worker pool**: main thread pushes jobs to channel, multiple worker threads pop and execute.
* **Fan-out / Fan-in**: split work across many threads then collect results with another channel.
* **Request/Response**: include a reply `Sender` in the request struct (`oneshot` pattern).

### Request/Response (oneshot) example

```rust
struct Request {
    data: String,
    reply: std::sync::mpsc::Sender<Response>,
}
```

---

## 6 — Shared state: `Arc`, `Mutex`, `RwLock`, `Atomic*`, `RefCell` caveats

### `Arc<T>` — share ownership across threads

Wrap `T` in `Arc<T>` to allow multiple threads to own it:

```rust
use std::sync::Arc;
let shared = Arc::new(MyState{});
let t = thread::spawn({
    let s = Arc::clone(&shared);
    move || { /* use s */ }
});
```

### `Mutex<T>` — mutual exclusion

Guard mutable shared state:

```rust
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(vec![]));
{
    let mut v = data.lock().unwrap();
    v.push(1);
}
```

* `lock()` blocks; returns `LockResult<MutexGuard<T>>` to handle poisoning.
* Prefer short critical sections; minimize work while holding lock.

### `RwLock<T>` — multiple readers / single writer

Use when reads far outnumber writes:

```rust
use std::sync::RwLock;
let r = RwLock::new(0);
{
    let read = r.read().unwrap(); // many readers allowed
}
{
    let mut write = r.write().unwrap(); // exclusive
}
```

### `Atomic*` types

* `AtomicBool`, `AtomicUsize`, `AtomicPtr`, `AtomicI64`, etc.
* Use for lock-free counters, flags, sequence numbers.
* Must pay attention to memory ordering: `Ordering::SeqCst`, `Acquire`, `Release`, `Relaxed`.
* `SeqCst` is simplest and safest; others give performance but require expertise.

Example:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: AtomicUsize = AtomicUsize::new(0);
COUNTER.fetch_add(1, Ordering::SeqCst);
```

### `RefCell`/`Cell` are NOT thread-safe

* `RefCell` and `Cell` are `!Sync` and `!Send`. They are single-threaded interior mutability tools. Do not use across threads.

---

## 7 — Condvar, Barrier, Once and synchronization patterns

### `Condvar` — condition variable

Use with a Mutex to wait for some condition:

```rust
use std::sync::{Mutex, Condvar, Arc};
let pair = Arc::new((Mutex::new(false), Condvar::new()));
// thread waits
let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
while !*started { started = cvar.wait(started).unwrap(); }
```

### `Barrier`

All threads wait until N threads reach barrier:

```rust
use std::sync::Arc;
use std::sync::Barrier;
let b = Arc::new(Barrier::new(10));
for _ in 0..10 {
    let c = Arc::clone(&b);
    thread::spawn(move || {
        // do work
        c.wait();
    });
}
```

### `Once`

Run initialization exactly once (thread-safe):

```rust
use std::sync::Once;
static INIT: Once = Once::new();
INIT.call_once(|| { /* init */ });
```

---

## 8 — Parking & thread control

### `thread::park` / `thread::unpark`

* A thread can call `park()` to block itself until another thread calls `unpark()` on its `Thread` handle.
* Useful for building low-level primitives or event reactors.

```rust
let parked = std::thread::spawn(|| {
    std::thread::park();
    println!("unparked!");
});
std::thread::sleep(std::time::Duration::from_millis(100));
parked.thread().unpark();
```

### parking_lot crate

* `parking_lot` provides faster `Mutex`, `RwLock`, `Condvar` implementations with less poisoning overhead and better performance characteristics. Widely used in production.

Use `parking_lot` if you need performance or more ergonomic APIs. Example:

```toml
# Cargo.toml
parking_lot = "0.12"
```

---

## 9 — Thread pools and Rayon

### Thread pool basics

* Use a thread pool when you have many small tasks to avoid thread creation cost.
* Create a job queue and workers (or use existing libs: `rayon`, `threadpool`, `tokio` runtime for async tasks).

Example libraries:

* `rayon`: data-parallelism, easy parallel iterators.
* `tokio` / `async-std`: async runtimes with worker thread pools.

### Rayon

Use `rayon` to parallelize CPU-bound loops simply:

```rust
use rayon::prelude::*;
let sum: i32 = (0..1000000).into_par_iter().map(|x| x*x).sum();
```

Rayon handles work stealing and scheduling.

---

## 10 — Async vs threads — what to use when

### Threads

* Use for CPU-bound tasks where you want parallel execution across cores.
* Good when tasks are blocking I/O or you need OS threads (e.g., FFI).

### Async (futures) with runtimes

* Use for high-concurrency I/O-bound workloads where tasks spend much time waiting (network, disk).
* Single-threaded async can handle thousands of concurrent I/O tasks with small memory per task.
* Requires async runtimes (Tokio, async-std). Async is cooperative: tasks yield explicitly (via `.await` or other suspension points).

### Interoperability

* You can mix: spawn blocking tasks in async runtimes (e.g., `tokio::task::spawn_blocking`) or run async in threads with `tokio::runtime::Runtime`.

**Rule of thumb**:

* If you’re writing an HTTP server serving many connections and most time is I/O → async.
* If you’re doing heavy CPU work (image processing, compression) → threads or Rayon for parallel loops.
* If in doubt and workload is moderate → threads are simpler. For massive concurrency and I/O, learn async.

---

## 11 — Lock-free structures & atomics (guidelines and pitfalls)

* Use atomics for simple shared counters, flags, or lock-free queues (but be careful).
* Implementing correct lock-free data structures is very tricky — prefer well-tested crates.
* Atomics do not provide higher-level invariants (e.g., atomic swap of multiple fields) — may need to combine with locking.

Memory ordering:

* `Ordering::Relaxed` — only atomicity, no synchronization
* `Ordering::Acquire` / `Release` — synchronize loads/stores
* `Ordering::SeqCst` — global sequential consistency; safest but slower.

When designing lock-free algorithms, document the ordering scheme and test thoroughly.

---

## 12 — Actor model and message-driven patterns

* Actors encapsulate state and only mutate via message handlers. They avoid shared-memory locks.
* Implement with channels and a worker thread (or async tasks).
* Benefits: encapsulation, easier reasoning, natural for distributed systems.

Example minimal actor:

```rust
use std::sync::mpsc;
use std::thread;

fn actor(rx: mpsc::Receiver<String>) {
    for msg in rx { println!("actor got {}", msg); }
}
let (tx, rx) = mpsc::channel();
thread::spawn(move || actor(rx));
tx.send("hello".into()).unwrap();
```

---

## 13 — Deadlocks, livelocks, poisoning, and avoidance

### Deadlocks

* Happens when two or more threads wait on locks the other holds.
* Avoid by:

  * Lock ordering: acquire multiple locks in a fixed global order.
  * Minimize lock scope.
  * Prefer fine-grained locking or message passing.

### Livelock

* Threads keep yielding but no progress. Avoid by backoff strategies and fairness.

### Poisoning

* Rust `Mutex` becomes poisoned on panic; handle `PoisonError` consciously (recover or abort).
* `parking_lot` avoids poisoning semantics.

---

## 14 — Testing & debugging concurrent code

### Deterministic testing

* Use single-threaded deterministic logic where possible.
* Use `loom` for model checking: explores possible interleavings in concurrent code — great for low-level concurrency. (Highly recommended for library authors.)

### Tools

* `println!` and logs — easy but noisy.
* `env_logger` or `tracing` crates for structured logging.
* `perf`, `async-profiler`, or `tokio-console` for async workload visualization.
* `addr2line`, `gdb`, `rr` for debugging.

### Strategies

* Unit test small components.
* Use integration tests for full interactions.
* Reproduce race conditions with stress tests (loops + randomized delays).

---

## 15 — Performance: microbenchmarking concurrency

* Use `criterion` for micro-benchmarks.
* Measure under realistic load (release mode).
* Test with various numbers of threads; measure scalability and contention.
* Profile locking hotspots (e.g., if mutex wait time dominates).
* Consider lock-free or per-thread buffers (sharding) if contention is high.

---

## 16 — Production patterns & architecture examples

### Pattern: Shared Command Queue

* Producer(s) push commands into channel(s).
* Worker pool processes commands, updates state through actor-owned state or locked resources.

### Pattern: Read-mostly service

* Use `Arc<RwLock<T>>` with many readers, occasional writer.

### Pattern: Sharded counters

* Use per-thread counters (local) and occasional aggregation to reduce hot spots.

### Pattern: Work-stealing pool

* Use Rayon for divide-and-conquer parallelism.

### Pattern: Async network server

* Use Tokio; run I/O handlers as async tasks; offload CPU-bound tasks to `spawn_blocking` or Rayon.

---

## 17 — Exercises & project roadmap (graded, progressive)

### Beginner (learn fundamentals)

1. Spawn 4 threads; each prints a message and returns a number; `join()` and sum results. (score: correctness & panic handling)
2. Implement a thread-safe counter using `Arc<Mutex<i32>>`. Test with 10 threads incrementing 1000 times. (measure final value)

### Intermediate (practical patterns)

3. Build a worker pool: main thread sends 100 jobs to channel; 4 workers process jobs; use `crossbeam::channel`. Report throughput.
4. Implement a `Barrier` usage: N threads do partial work, then barrier, then next phase.

### Advanced (real systems)

5. Implement an actor system with typed messages and supervisor that restarts crashed actors. (graded on safety & restart logic)
6. Create a parallel map-reduce using Rayon and compare with manual thread pool implementation. Benchmark both.

### Expert (research-grade)

7. Use `loom` to comprehensively test a custom lock-free queue (or a small concurrent ring buffer). Document invariants & memory order usage.
8. Build an async HTTP proxy with Tokio that forwards requests and caches responses using `Arc<RwLock<HashMap>>` and a background cleaner. Benchmark throughput and latency.

---

## 18 — Further reading & recommended crates

* Crates:

  * `crossbeam` (channels, scoped threads, epoch GC, etc.)
  * `parking_lot` (fast locks)
  * `rayon` (data parallelism)
  * `tokio` / `async-std` (async runtimes)
  * `loom` (concurrency testing/model checking)
  * `flume` (channels alternative)
  * `tracing` / `tracing-subscriber` (structured logging)
* Concepts/reading:

  * "The Rustonomicon" (unsafe and concurrency pitfalls)
  * "Rust Concurrency Book" (community resources)
  * Academic: memory models and weak ordering (if you use advanced atomics)
  * Kernel/OS-level threading guides for deep understanding of scheduling and blocking costs

---

## Appendix — Handy code snippets (copy-paste)

### Worker pool (compact)

```rust
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::thread;

fn start_pool(size: usize) -> (Sender<Box<dyn FnOnce() + Send + 'static>>, Vec<thread::JoinHandle<()>>) {
    let (tx, rx) = unbounded::<Box<dyn FnOnce() + Send + 'static>>();
    let mut handles = Vec::new();
    for _ in 0..size {
        let r = rx.clone();
        handles.push(thread::spawn(move || {
            for job in r.iter() {
                job();
            }
        }));
    }
    (tx, handles)
}
```

### Scoped threads example

```rust
fn main() {
    let mut v = vec![1,2,3];
    std::thread::scope(|s| {
        s.spawn(|| {
            // short-lived work
            println!("inside scope");
        });
    });
    println!("v still available: {:?}", v);
}
```

### Park & unpark example

```rust
let handler = std::thread::spawn(|| {
    println!("parking");
    std::thread::park();
    println!("unparked");
});
std::thread::sleep(std::time::Duration::from_millis(100));
handler.thread().unpark();
```

---

## Wrap-up: Practical checklist before shipping concurrent Rust code

1. Are invariants documented? (who owns what, who may mutate, lifetime)
2. Are shared data structures minimized and encapsulated behind clear APIs?
3. Are blocking operations offloaded to pools or separate threads?
4. Do you prefer message passing where appropriate?
5. Are locks held for minimal durations and in consistent order?
6. Are panics & poison handled sensibly (supervisor patterns)?
7. Are you measuring & profiling under realistic load?
8. Are unit tests + stress tests + `loom` checks included where appropriate?

---

If you want, I’ll next:

* Expand any **single section** into a full chapter with deeper examples and unit tests (e.g., a complete worker pool with graceful shutdown and metrics).
* Generate full working **projects** (worker pool, actor system, async proxy) as copy-paste-ready Cargo projects with tests and benchmarks.
* Produce a graded assignment & rubric for one of the exercises and grade your submission.

Which do you want to build next?
