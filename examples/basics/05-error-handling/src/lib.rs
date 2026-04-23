//! # Panic vs Error Handling
//!
//! Demonstrates when to use panic! vs Result<T, Error> in Soroban contracts.
//!
//! ## Key Principles
//!
//! **Use Result<T, Error> for:**
//! - Expected failures (validation, business logic)
//! - Recoverable conditions
//! - User input errors
//! - Better gas efficiency (no stack unwinding)
//!
//! **Use panic! for:**
//! - Invariant violations
//! - Unreachable code paths
//! - Critical internal errors
//! - Development/debugging assertions

#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InsufficientBalance = 2,
    Unauthorized = 3,
}

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

#[contract]
pub struct ErrorHandlingContract;

#[contractimpl]
impl ErrorHandlingContract {
    fn validate_transfer(amount: u64, balance: u64) -> Result<(), Error> {
        if amount == 0 {
            return Err(Error::InvalidAmount);
        }
        if amount > balance {
            return Err(Error::InsufficientBalance);
        }
        Ok(())
    }

    fn subtract_balance(amount: u64, balance: u64) -> Result<u64, Error> {
        balance.checked_sub(amount).ok_or(Error::InvalidAmount)
    }

    /// ✅ GOOD: Use Result for expected validation failures
    /// Returns error for invalid input - caller can handle gracefully
    pub fn transfer(amount: u64, balance: u64) -> Result<u64, Error> {
        Self::validate_transfer(amount, balance)?;
        Self::subtract_balance(amount, balance)
    }

    /// ❌ BAD: Panic for expected validation (anti-pattern)
    /// Panics waste gas and provide poor UX
    pub fn transfer_panic(amount: u64, balance: u64) -> u64 {
        if amount == 0 {
            panic!("invalid amount");
        }
        if amount > balance {
            panic!("insufficient balance");
        }
        balance - amount
    }

    /// ✅ GOOD: Panic for invariant violations
    /// Internal state should never be invalid - panic is appropriate
    pub fn get_verified_state(env: Env, key: u32) -> u64 {
        let value: u64 = env.storage().instance().get(&key).unwrap_or(0);
        // Invariant: value must be <= 1000 (enforced by all setters)
        if value > 1000 {
            panic!("invariant violated: state corrupted");
        }
        value
    }

    /// ✅ GOOD: Result for business logic errors
    /// Division by zero is expected user error, not a bug
    pub fn divide(a: i128, b: i128) -> Result<i128, Error> {
        Ok(Self::divide_checked(a, b).map_err(Error::from)?)
    }

    /// Core division operation returning a domain-specific error type.
    fn divide_checked(a: i128, b: i128) -> Result<i128, MathError> {
        if b == 0 {
            return Err(MathError::DivisionByZero);
        }
        Ok(a / b)
    }

    /// Converts lower-level math errors into contract-level errors.
    pub fn divide_with_conversion(a: i128, b: i128) -> Result<i128, Error> {
        Ok(Self::divide_checked(a, b).map_err(Error::from)?)
    }
}

#[cfg(test)]
mod test;
