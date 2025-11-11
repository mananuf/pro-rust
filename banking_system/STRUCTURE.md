```pgsql
.
├── Cargo.toml           # workspace
|__ bin/
    │── Cargo.toml
    └── src/main.rs
|__crates/
|____
    ├── core/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs       # public domain types, traits
    │       └── account.rs
    |       |__ customer.rs
    ├── service/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs       # orchestrates domain + infra via traits
    │       └── bank.rs
    ├── infra/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs       # concrete DB implementation, file or in-memory
    │       └── storage.rs
    └── tests/
        └── integration.rs
```