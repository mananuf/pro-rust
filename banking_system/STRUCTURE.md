```pgsql
.
├── Cargo.toml           # workspace
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs       # public domain types, traits
│       └── account.rs
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
├── api/
│   ├── Cargo.toml
│   └── src/main.rs     # CLI or HTTP server using `warp` / `axum`
└── tests/
    └── integration.rs
```