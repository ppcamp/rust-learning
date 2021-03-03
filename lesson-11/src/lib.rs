#![allow(dead_code)]
pub mod geometric {
  #[derive(Debug)]
  pub struct Rectangle {
    pub width: u32,
    pub height: u32,
  }

  impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
    }
  }
}

#[cfg(test)]
mod tests {
  use super::geometric::Rectangle;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    // Asser test for boolean
    assert!(larger.can_hold(&smaller));

    // assert_eq!(5, value); value is some string to print when failed
    // assert_eq! // test for equalty
    // assert_ne! // test for inequality
  }

  // #[ignore] the macro ignore will ignore the test unless its request in `cargo test test_name`
  // The code bellow is used to check for panic
  // #[test]
  // #[should_panic]
  // fn greater_than_100() {
  //     Guess::new(200);
  // }
}
