extern crate lesson_11;
pub use lesson_11::geometric::*;

#[test]
fn smaller_cant_hold_greater() {
  let larger = Rectangle {
    width: 8,
    height: 7,
  };
  let smaller = Rectangle {
    width: 5,
    height: 1,
  };

  assert!(!smaller.can_hold(&larger), format!("{:?}", larger));
}
