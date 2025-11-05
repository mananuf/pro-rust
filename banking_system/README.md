# Graded Banking System task (spec & rubric)

Below is a **complete assignment**. Implement it in a crate (or workspace). I’ll grade on correctness, tests, code quality, safety, and documentation.

---

## Task: **SimpleBank — a small banking system**

**Goal**: build a minimal but realistic banking system that supports accounts, customers, deposits, withdrawals, transfers, balances, and statements. Apply professional modularity and testing.

### Requirements (must implement)

1. **Domain types (core crate / module)**

   * `Customer { id: CustomerId, name: String }`
   * `Account { id: AccountId, owner: CustomerId, balance: Money, status: AccountStatus }`
   * `AccountStatus` enum: `Active`, `Frozen`, `Closed`
   * `Money` type alias or newtype (prefer newtype) supporting decimals (use `i128` cents or `rust_decimal` if you want)
   * `Transaction` enum: `Deposit(Money)`, `Withdraw(Money)`, `Transfer { to: AccountId, amount: Money }`

2. **Repository trait** (in `core`):

   ```rust
   pub trait AccountRepository {
       fn create(&self, account: Account) -> Result<(), RepoError>;
       fn get(&self, id: AccountId) -> Result<Option<Account>, RepoError>;
       fn update(&self, account: Account) -> Result<(), RepoError>;
   }
   ```

   Provide `InMemoryAccountRepo` in `infra` crate (with `Arc<RwLock<HashMap<...>>>`).

3. **Domain logic inside `Account`**:

   * `fn deposit(&mut self, amount: Money) -> Result<Money, DomainError>` returns new balance
   * `fn withdraw(&mut self, amount: Money) -> Result<Money, DomainError>`
   * `fn apply_transaction(&mut self, txn: Transaction) -> Result<Money, DomainError>`

   Enforce rules:

   * cannot operate on `Closed`
   * `Frozen` only allows deposits (no withdrawals/transfers)
   * no negative deposits/withdrawals
   * insufficient funds error on withdraw/transfer

   Define `DomainError` enum with descriptive variants.

4. **Bank service (application layer)**:

   * `fn create_account(&self, owner: CustomerId) -> Result<AccountId, AppError>`
   * `fn process(&self, account_id: AccountId, txn: Transaction) -> Result<Money, AppError>`
   * `fn transfer(&self, from: AccountId, to: AccountId, amount: Money) -> Result<(), AppError>` (atomic in repo sense if possible; for in-memory, lock both accounts in canonical order to avoid deadlocks)

   Service should depend on `AccountRepository` trait (injected).

5. **Errors**:

   * Use `thiserror` to define error enums for Domain (`DomainError`), Repo (`RepoError`), and App (`AppError`). Compose errors with `#[from]` where appropriate.

6. **Tests**:

   * Unit tests for `Account` methods (deposit/withdraw/apply_transaction).
   * Unit tests for `Bank` service using `InMemoryAccountRepo`.
   * Integration test performing multiple transfers and asserting final balances.
   * Edge-case tests (insufficient funds, frozen account, closed account, negative amounts).
   * Use `#[should_panic]` only when you expect panics (rarely). Prefer returning errors.

7. **Concurrency safety**:

   * In-memory repo must be safe for concurrent access: use `Arc<RwLock<HashMap<...>>>`.
   * `transfer` should avoid deadlocks: lock accounts in deterministic order (e.g., by `min(id)` first) if you lock both.

8. **CLI or example**:

   * Provide a small `examples/` or `src/bin/` showing create accounts, deposit, withdraw, transfer and printing results.

9. **(Optional / Bonus)**:

   * Implement interest accrual operation or scheduled payments.
   * Implement a statement history inside account (Vec of events).
   * Add benchmarks with `criterion` for deposit/withdraw/transfer.
   * Provide a small HTTP API using `axum` or `warp`.

---

## Deliverables

* A Git repo / folder with code organized (prefer workspace with `core`, `service`, `infra`, `api`).
* Unit tests and integration tests: `cargo test` must pass.
* README with usage and architecture explanation (1–2 paragraphs).
* Optional: benchmark file(s).

---

## Evaluation & grading (out of 10)

I will grade your submission according to:

1. **Correctness (3 pts)** — correct domain behavior, tests passing, errors handled.
2. **Design & Modularity (2 pts)** — separation of concerns, traits, layers, DI.
3. **Safety & Concurrency (1.5 pts)** — correct use of locks/Arc, no `unsafe`, avoids deadlocks.
4. **Error handling & types (1 pt)** — typed errors, `thiserror` usage, no swallowing.
5. **Tests & Coverage (1 pt)** — unit tests for domain, service tests, integration tests.
6. **Code quality & docs (1 pt)** — readability, comments, README, `cargo fmt` clean.

**Extra credit (up to +1 pt)**:

* Benchmarks and profiling results, or statement history, or interest scheduling.

Total: 10 + 1 bonus.

---

## Example API / Types (starter code snippets)

`core/src/types.rs`

```rust
pub type AccountId = u64;
pub type CustomerId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money(i128); // cents

impl Money {
    pub fn from_cents(c: i128) -> Self { Self(c) }
    pub fn cents(self) -> i128 { self.0 }
}
```

`core/src/account.rs`

```rust
#[derive(Debug, Clone)]
pub struct Account {
    pub id: AccountId,
    pub owner: CustomerId,
    pub balance: Money,
    pub status: AccountStatus,
}

impl Account {
    pub fn deposit(&mut self, amount: Money) -> Result<Money, DomainError> { ... }
    pub fn withdraw(&mut self, amount: Money) -> Result<Money, DomainError> { ... }
    pub fn apply_transaction(&mut self, txn: Transaction) -> Result<Money, DomainError> { ... }
}
```

`infra/src/in_memory_repo.rs`

```rust
pub struct InMemoryRepo {
    store: Arc<RwLock<HashMap<AccountId, Account>>>,
}
impl AccountRepository for InMemoryRepo { ... }
```

`service/src/bank.rs`

```rust
pub struct Bank<R: AccountRepository> {
    repo: Arc<R>,
}
impl<R: AccountRepository> Bank<R> {
    pub fn transfer(&self, from: AccountId, to: AccountId, amount: Money) -> Result<(), AppError> {
        // lock ordering, get accounts, apply, update
    }
}
```

---

## How to submit

* Paste your code (or a link to a GitHub gist / repo).
* Include `cargo test` output.
* Include short README with commands to run.

When you submit, I will:

1. Run tests (conceptually) and inspect code.
2. Grade it using the rubric above.
3. Provide line-by-line improvements, refactor suggestions, and optional performance tips.
4. Optionally refactor code to idiomatic form and provide PR-like diffs.

---

## Example small checklist to follow when implementing

* [ ] `core` crate: types, errors, domain
* [ ] `infra` crate: InMemory repo
* [ ] `service` crate: Bank wiring using trait
* [ ] tests/unit for `Account`
* [ ] tests/service using `InMemoryRepo`
* [ ] README & example run
* [ ] Optional: benches, CLI


