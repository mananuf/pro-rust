### In production-grade Rust systems:
---
- Use type aliases for developer ergonomics.
- Use newtypes for business logic correctness.
- Both are zero-cost, but only one provides zero-risk.

### Production Rule of Thumb

| Scenario                          | Use            | Reason                                      |
| --------------------------------- | -------------- | ------------------------------------------- |
| Long or complex generics          | **Type Alias** | Makes code readable                         |
| Same base type, different meaning | **Newtype**    | Enforces domain distinction                 |
| Internal module shortcuts         | **Type Alias** | Keeps APIs concise                          |
| Financial / safety-critical logic | **Newtype**    | Prevents mixing semantically different data |
| Need trait implementations        | **Newtype**    | Aliases can’t implement traits              |
| Performance-sensitive systems     | Both           | Equal efficiency                            |

### When to Use a Newtype

Think of newtypes as ID cards.
Two people might both be named “John”, but their IDs are not interchangeable.

Use it:

* When you want semantic safety between same underlying types.
* To enforce invariants or constraints.
* When modeling domain-specific values (finance, users, units, etc.).
* When creating custom traits or type-specific methods.

### Example: Financial Systems (Avoiding Costly Bugs)

#### ❌ Bad: No distinction between currencies

```rust
fn pay(amount: f64, account_id: u64) {
    println!("Paying {} to {}", amount, account_id);
}

let usd_balance: f64 = 100.0;
let ngn_balance: f64 = 100000.0;

pay(ngn_balance, 5); // Mistakenly sends Naira instead of USD!
```

#### ✅ Good: Strongly typed newtypes

```rust
struct USD(f64);
struct NGN(f64);

fn pay(amount: USD, account_id: u64) {
    println!("Paying ${} to {}", amount.0, account_id);
}

let usd_balance = USD(100.0);
let ngn_balance = NGN(100000.0);

// pay(ngn_balance, 5); ❌ Compile-time error
pay(usd_balance, 5);   // ✅ Correct
```

### When to Use a Type Alias

Think of type alias like a nickname.
It’s still the same person — you just call them differently.

Use it:

✅ To simplify long or complex type names.

✅ To improve readability without changing behavior.

✅ When type distinction doesn’t matter semantically.

### Example:
#### ❌ Before (too verbose)
```rust
use std::collections::HashMap;

fn get_user_roles() -> HashMap<String, Vec<String>> {
    HashMap::new()
}
```

#### ✅ After (type alias)
```rust
type UserRoles = HashMap<String, Vec<String>>;

fn get_user_roles() -> UserRoles {
    HashMap::new()
}
```