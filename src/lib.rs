//! # Eiffel Inspired Invariant Checking
//!
//! This crate provides a set of macros inspired by the Eiffel programming language's
//! features for invariant checking. These features include checks for loops, entry, exit, and more.
//!
//! The Eiffel language's options for invariant checking serve as the basis for the design
//! and functionality of the macros in this crate.
//!
//! Please note that this crate is still a work in progress. As such, some features may not be fully
//! implemented or may undergo significant changes in future updates.
//!
//! Contributions and feedback are always welcome.
#![deny(warnings)]
#![deny(missing_docs)]

use thiserror::Error;

/// A simple error type to represent a precondition failed error.
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct PreconditionFailedError {
    message: &'static str,
}

#[macro_export]
/// `require` is a macro that checks if a given condition is met.
/// If the condition is not met, the macro will cause the program to panic with a specified message.
/// 
/// # Arguments
/// 
/// * `$condition`: An expression that should evaluate to a boolean. This is the precondition that needs to be checked.
/// * `$msg`: A message that will be printed if the precondition is not met.
/// 
/// # Panics
/// 
/// The macro panics if the precondition `$condition` is not met, with a panic message of the form: "Precondition failed: $msg".
macro_rules! require {
    ($condition:expr, $msg:expr) => {
        if !$condition {
            panic!("Precondition failed: {}", $msg);
        }
    };
}

#[macro_export]
/// `require_or_err` is a macro that checks if a given condition is met.
/// If the condition is not met, the macro will return an error of type `PreconditionFailedError` with a specified message.
/// 
/// # Arguments
/// 
/// * `$condition`: An expression that should evaluate to a boolean. This is the precondition that needs to be checked.
/// * `$msg`: A message that will be included in the `PreconditionFailedError` if the precondition is not met.
/// 
/// # Errors
/// 
/// The macro returns an error of type `PreconditionFailedError` if the precondition `$condition` is not met, with an error message of the form: "Precondition failed: $msg".
macro_rules! require_or_err {
    ($condition:expr, $msg:expr) => {
        if !$condition {
            return Err($crate::PreconditionFailedError { message: $msg }.into());
        }
    };
}

#[cfg(test)]
mod tests {
    ///! Basic usage of the `require!` and `require_or_err!` macros.
    use super::*;

    fn example_function_with_result(x: i32) -> Result<(), PreconditionFailedError> {
        require_or_err!(x > 0, "x must be greater than 0");

        // Proceed with the function logic
        println!("x is a valid argument: {}", x);
        Ok(())
    }

    fn example_function_without_result(x: i32) {
        require!(x > 0, "x must be greater than 0");
    
        // Proceed with the function logic
        println!("x is a valid argument: {}", x);
    }

    
    #[test]
    fn it_works() {
        assert!(example_function_with_result(1).is_ok());
        assert!(example_function_with_result(0).is_err());
        assert!(example_function_with_result(-1).is_err());
    }

    #[test]
    fn it_works_without_result() {
        example_function_without_result(1);
    }

    #[test]
    #[should_panic]
    fn it_works_without_result_and_panics() {
        example_function_without_result(0);
    }
}
