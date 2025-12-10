## Patterns
`$x:expr` → expression

`$x:ident` → identifier

`$x:ty` → type

`$x:block` → block

`$x:pat` → pattern

`$($x:tt)*` → token tree repetition

## Common Declarative Macro Patterns
### Pattern 1 — Variadic argument macros

```rust
    ($($arg:expr),*) => { ... }
```

### Pattern 2 — DSL Construction

```rust
    route! {
        GET /home => home_handler,
        POST /login => login_handler
    }
```

## Procedural Macros

Procedural macros are compiled as a separate crate of type:

```rust 
    [lib]
    proc-macro = true
```


They receive a token stream (TokenStream) and produce another.

### Types of Procedural Macros

1. Derive macros
```rust
    #[derive(MyTrait)]
    struct A;
```

2. Attribute macros
```rust
    #[route(GET, "/home")]
    fn home() {}
```

3. Function-like macros
```rust
    sql!(SELECT * FROM users WHERE id = 1);
```

# THE COMPLETE BREAKDOWN OF DECLARATIVE MACROS IN RUST

_(Everything about fragment specifiers & when to use them)_

Declarative macros are syntax-pattern matchers.

They match Rust tokens, not types.

## SECTION 1 — The Big Idea
A declarative macro has 3 parts:
```rust
macro_rules! name {
    ( pattern ) => { expansion };
}
```

Patterns are built using `$var:fragment_specifier`.

## SECTION 2 — All Fragment Specifiers (The 15 Needed in Real Rust)

Rust has many "matchers". These are the most important ones:

| Specifier      | What it matches      | Typical usage                  |
| -------------- | -------------------- | ------------------------------ |
| **`ident`**    | identifiers          | variable names, function names |
| **`expr`**     | expressions          | arithmetic, function calls     |
| **`ty`**       | types                | generics, struct fields        |
| **`path`**     | type or module paths | traits, structs, modules       |
| **`pat`**      | patterns             | match arms, destructuring      |
| **`stmt`**     | a statement          | inside functions               |
| **`block`**    | a block `{}`         | inline code blocks             |
| **`item`**     | a full item          | structs, enums, impls          |
| **`meta`**     | meta attributes      | #[derive(...)]                 |
| **`tt`**       | arbitrary token tree | the most flexible              |
| **`literal`**  | literals             | numbers, strings               |
| **`vis`**      | visibility           | pub, pub(crate)                |
| **`lifetime`** | lifetime names       | 'a                             |
| **`path`**     | paths                | std::io, A::B                  |
| **`type`**     | alias for ty         | works similarly                |

## SECTION 3 — SPECIFIER-BY-SPECIFIER DEEP DIVE

Each section includes:

-    What it matches
-    Why you use it
-    Good & bad usage
-    Exercises


### 1. ident — Identifiers

Matches names only.

✔ Use when you need to generate:

- Function names

- Variable names

- Struct or enum names

✔ Example:
```rust
    macro_rules! make_var {
        ($name:ident) => {
            let $name = 10;
        };
    }

    fn main() {
        make_var!(score);
        println!("{}", score); // 10
    }
```

### 2. ty — Types

Matches type positions.

✔ Matches:

- i32

- Vec<String>

- Option<&'a str>

```rust
    macro_rules! define_container {
        ($type:ty) => {
            struct Container {
                value: $type
            }
        };
    }

    define_container!(Vec<String>);
```