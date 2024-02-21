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

// Re-export macros from eiffel-macros submodule
pub use eiffel_macros::*;

// Re-export macros from eiffel-gen submodule
pub use eiffel_macros_gen::*;

#[cfg(test)]
mod tests {
  // use super::*;
  use eiffel_macros_gen::check_invariant;

  struct MyClass {
    // Fields
    a: i32,
  }

  impl MyClass {
    fn my_invariant(&self) -> bool {
      // Your invariant checks here
      self.a > 0
    }

    #[check_invariant(my_invariant)]
    fn my_method(&mut self, value_to_add: i32) {
      // Method body
      self.a += value_to_add;
      // println!("Method body {:?}", self.a);
    }

    #[check_invariant(my_invariant, "before")]
    fn my_method_before_only(&mut self, value_to_add: i32) {
      // Method body
      self.a += value_to_add;
      // println!("Method body {:?}", self.a);
    }

    #[check_invariant(my_invariant, "after")]
    fn my_method_after_only(&mut self, value_to_add: i32) {
      // Method body
      self.a += value_to_add;
      // println!("Method body {:?}", self.a);
    }
  }       

  #[test]
  #[should_panic]
  fn test_changing_it_to_an_invalid_value() {
      let mut my_class = MyClass {
        a: 1
      };

      my_class.my_method(-2);
  }
  
  #[test]
  #[should_panic]
  fn test_already_wrong() {
    let mut my_class = MyClass {
      a: -1
    };

    my_class.my_method(2);

    assert_eq!(my_class.a, 1);
  }

  #[test]
  fn test_only_check_before_only() {
    let mut my_class = MyClass {
      a: 1
    };

    // Would panic if the check was after the method call
    my_class.my_method_before_only(-2);

    assert_eq!(my_class.a, -1);
  }

  #[test]
  fn test_only_check_after_only() {
    let mut my_class = MyClass {
      a: -1
    };

    // Would panic if the check was after the method call
    my_class.my_method_after_only(2);

    assert_eq!(my_class.a, 1);
  }
}