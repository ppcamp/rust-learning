// externing crate for test-only use

// The code bellow change the message in assert macros (colorize and show diff)
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// Function to test
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

// Test itself
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(self::add(2, 2), 5);
  }
}
