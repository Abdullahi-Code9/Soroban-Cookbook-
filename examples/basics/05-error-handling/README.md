# Error Handling

<<<<<<< HEAD
Demonstrates proper error handling in Soroban smart contracts: when to use `Result<T, Error>` and when `panic!` is the right choice.

## Overview

This contract compares two approaches to failure handling side-by-side:

- **`Result<T, Error>`** — the preferred pattern for expected, recoverable failures such as invalid user input or insufficient funds. Cheaper in gas (no stack unwinding) and composable with Rust's `?` operator.
- **`panic!`** — appropriate only for invariant violations and unreachable code paths where continuing execution would leave the contract in a corrupt state.

Understanding the difference is foundational to writing production-quality Soroban contracts.
=======
This example demonstrates proper error propagation patterns in Soroban smart contracts using custom error types, `Result<T, E>`, the `?` operator, and explicit error conversion.
>>>>>>> 14a4b82 (Implement explicit error propagation patterns.)

## Project Structure

```text
05-error-handling/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs      # contract and error definitions
    └── test.rs     # unit tests across five test categories
```

<<<<<<< HEAD
## Key Concepts

### `#[contracterror]`
=======
## What This Example Shows

- Defining contract-level and domain-level error enums
- Returning `Result<T, Error>` for recoverable failures
- Propagating errors with the `?` operator across helper functions
- Converting lower-level errors into contract errors with `From`
- Verifying bubbling behavior and conversion in tests

## Key Concepts

### Contract Error Type
>>>>>>> 14a4b82 (Implement explicit error propagation patterns.)

The `#[contracterror]` attribute transforms a plain Rust enum into a Soroban-compatible error type. Each variant maps to a stable `u32` code surfaced to callers across the host–guest boundary.

```rust
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
<<<<<<< HEAD
    InvalidAmount       = 1,
    InsufficientBalance = 2,
    Unauthorized        = 3,
}
```

Rules when defining contract errors:
=======
    InvalidAmount = 1,
    InsufficientBalance = 2,
    Unauthorized = 3,
}
```

### Domain Error + Conversion

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MathError {
    DivisionByZero = 10,
}

impl From<MathError> for Error {
    fn from(value: MathError) -> Self {
        match value {
            MathError::DivisionByZero => Error::InvalidAmount,
        }
    }
}
```

### Error Propagation with `?`
>>>>>>> 14a4b82 (Implement explicit error propagation patterns.)

- Always use `#[repr(u32)]`. The host encodes errors as `u32` values.
- Assign explicit discriminants starting at `1`. Zero is reserved by the host.
- Never change or reuse a discriminant after deployment. Callers and tooling depend on stable codes across upgrades.
- Derive `Copy + Clone + Eq + PartialEq` so errors can be compared in tests and match arms.

### `Result<T, Error>`

Any contract function that can fail for an expected reason should return `Result<T, Error>`. The host propagates the error code to the caller; the caller can branch on it or convert it to a host trap with `.unwrap()`.

## Code Walkthrough

### `transfer` — Result for validation failures

```rust
pub fn transfer(amount: u64, balance: u64) -> Result<u64, Error> {
<<<<<<< HEAD
    if amount == 0 {
        return Err(Error::InvalidAmount);
    }
    if amount > balance {
        return Err(Error::InsufficientBalance);
    }
    Ok(balance - amount)
=======
    Self::validate_transfer(amount, balance)?;
    Self::subtract_balance(amount, balance)
}
```

### Error Conversion and Bubbling

```rust
pub fn divide_checked(a: i128, b: i128) -> Result<i128, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

pub fn divide_with_conversion(a: i128, b: i128) -> Result<i128, Error> {
    Ok(Self::divide_checked(a, b).map_err(Error::from)?)
>>>>>>> 14a4b82 (Implement explicit error propagation patterns.)
}
```

Both failure conditions are expected user errors. Returning `Err` lets the caller decide how to respond without wasting gas on an unwinding panic.

<<<<<<< HEAD
### `transfer_panic` — panic as an anti-pattern

```rust
pub fn transfer_panic(amount: u64, balance: u64) -> u64 {
    if amount == 0 {
        panic!("invalid amount");
    }
    if amount > balance {
        panic!("insufficient balance");
    }
    balance - amount
}
```

Included to illustrate what **not** to do. Panicking on user input aborts the transaction and wastes all gas consumed up to that point. The caller has no path to handle the failure.

### `get_verified_state` — panic for invariant violations

```rust
pub fn get_verified_state(env: Env, key: u32) -> u64 {
    let value: u64 = env.storage().instance().get(&key).unwrap_or(0);
    // Invariant: value must be <= 1000 (enforced by all setters)
    if value > 1000 {
        panic!("invariant violated: state corrupted");
    }
    value
}
```

If storage holds a value above `1000`, every setter that ran previously violated the contract invariant — the contract is in an unrecoverable state. Panicking here is correct: it halts execution and signals that intervention is required.

### `divide` — Result for expected arithmetic errors

```rust
pub fn divide(a: i128, b: i128) -> Result<i128, Error> {
    if b == 0 {
        return Err(Error::InvalidAmount);
    }
    Ok(a / b)
}
```

Division by zero is a foreseeable user error, not a bug. Returning `Err` keeps the transaction alive and gives callers a recoverable path.

## Best Practices

| Scenario | Pattern | Reason |
| -------- | ------- | ------ |
| User supplies `amount = 0` | `Result` | Expected validation failure; caller should handle it |
| User supplies `amount > balance` | `Result` | Business logic error; recoverable |
| Internal storage value violates an invariant | `panic!` | Unrecoverable state; must abort |
| Unreachable branch hit at runtime | `unreachable!()` | Should never happen; signals a bug |
| Division by zero from user input | `Result` | Expected; caller can retry |

**Use `Result` when:**

- The failure is caused by user input or external state.
- The caller can meaningfully recover or retry.
- You want to preserve remaining gas for the rest of the transaction.

**Use `panic!` when:**

- An internal invariant has been violated.
- Continuing execution would produce incorrect results or corrupt state.
- The branch is logically unreachable.

## Custom Error Guide

### Defining error variants

```rust
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidAmount       = 1,  // zero or negative input
    InsufficientBalance = 2,  // not enough funds
    Unauthorized        = 3,  // caller lacks permission
}
```

### Returning errors from functions

```rust
pub fn withdraw(amount: u64, balance: u64, caller: Address, admin: Address)
    -> Result<u64, Error>
{
    if caller != admin {
        return Err(Error::Unauthorized);
    }
    if amount == 0 {
        return Err(Error::InvalidAmount);
    }
    if amount > balance {
        return Err(Error::InsufficientBalance);
    }
    Ok(balance - amount)
}
```

### Propagating errors with `?`

# Error Handling

This example demonstrates proper error propagation and handling patterns in Soroban smart contracts. It shows how to:

- Define contract-level errors using `#[contracterror]`.
- Return `Result<T, Error>` for expected, recoverable failures.
- Propagate errors with the `?` operator.
- Convert domain errors into contract errors with `From` implementations.
- Use `panic!` only for unrecoverable invariant violations.

## Project Structure

```text
05-error-handling/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs      # contract and error definitions
    └── test.rs     # unit tests across categories
```

## What This Example Shows

- Defining contract-level and domain-level error enums
- Returning `Result<T, Error>` for recoverable failures
- Propagating errors with the `?` operator across helper functions
- Converting lower-level errors into contract errors with `From`
- Verifying bubbling behavior and conversion in tests

## Key Concepts

### Contract Error Type

The `#[contracterror]` attribute transforms a Rust enum into a Soroban-compatible error type. Each variant maps to a stable `u32` code surfaced to callers.

```rust
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InsufficientBalance = 2,
    Unauthorized = 3,
}
```

Rules when defining contract errors:

- Use `#[repr(u32)]` and explicit discriminants (start at 1).
- Never renumber or reuse discriminants after deployment.
- Derive `Copy + Clone + Eq + PartialEq` for testability.

### Domain Error & Conversion

You can define domain (internal) errors and convert them into contract errors:

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MathError {
    DivisionByZero = 10,
}

impl From<MathError> for Error {
    fn from(value: MathError) -> Self {
        match value {
            MathError::DivisionByZero => Error::InvalidAmount,
        }
    }
}
```

### Error Propagation with `?`

Functions that return `Result` can use the `?` operator to forward errors:

```rust
pub fn divide_with_conversion(a: i128, b: i128) -> Result<i128, Error> {
    Ok(Self::divide_checked(a, b).map_err(Error::from)?)
}
```

### When to Use `Result` vs `panic!`

- Use `Result` for expected, recoverable failures caused by user input or external state (e.g., invalid amount, insufficient balance).
- Use `panic!` only for invariant violations or unrecoverable internal errors where continuing would corrupt state.

## Code Walkthrough

### `transfer` — return `Result` for validation failures

```rust
pub fn transfer(amount: u64, balance: u64) -> Result<u64, Error> {
    if amount == 0 {
        return Err(Error::InvalidAmount);
    }
    if amount > balance {
        return Err(Error::InsufficientBalance);
    }
    Ok(balance - amount)
}
```

### `divide_checked` and conversion

```rust
pub fn divide_checked(a: i128, b: i128) -> Result<i128, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

pub fn divide_with_conversion(a: i128, b: i128) -> Result<i128, Error> {
    Ok(Self::divide_checked(a, b).map_err(Error::from)?)
}
```

### `transfer_panic` — illustrative anti-pattern

This function demonstrates why panicking on expected user input is discouraged (it aborts the transaction and wastes gas):

```rust
pub fn transfer_panic(amount: u64, balance: u64) -> u64 {
    if amount == 0 {
        panic!("invalid amount");
    }
    if amount > balance {
        panic!("insufficient balance");
    }
    balance - amount
}
```

### `get_verified_state` — panic for invariant violations

Panicking is appropriate when an internal invariant is violated and manual intervention is required:

```rust
pub fn get_verified_state(env: Env, key: u32) -> u64 {
    let value: u64 = env.storage().instance().get(&key).unwrap_or(0);
    if value > 1000 {
        panic!("invariant violated: state corrupted");
    }
    value
}
```

## Best Practices

| Scenario | Pattern | Reason |
| -------- | ------- | ------ |
| User supplies `amount = 0` | `Result` | Expected validation failure; caller should handle it |
| User supplies `amount > balance` | `Result` | Business logic error; recoverable |
| Internal storage violates invariant | `panic!` | Unrecoverable; abort and investigate |
| Unreachable branch hit | `unreachable!()` | Signals a bug in logic |
| Division by zero from user input | `Result` | Caller can handle/avoid it |

## Testing Errors Guide

`src/test.rs` includes tests covering success paths, expected errors, panic scenarios, and conversion behavior.

### Examples

```rust
#[test]
fn test_transfer_success() {
    assert_eq!(ErrorHandlingContract::transfer(50, 100), Ok(50));
}

#[test]
fn test_transfer_invalid_amount_zero() {
    assert_eq!(ErrorHandlingContract::transfer(0, 100), Err(Error::InvalidAmount));
}

#[test]
fn test_divide_by_zero() {
    assert_eq!(ErrorHandlingContract::divide(10, 0), Err(Error::InvalidAmount));
}
```

## Conclusion

Prefer `Result` for recoverable, expected failures and reserve `panic!` for true invariants and unrecoverable conditions. Keep error discriminants stable across upgrades and convert domain errors to contract errors explicitly.
Tests in `src/test.rs`:
