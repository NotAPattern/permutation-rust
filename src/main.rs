mod permutation;

use std::cell::RefCell;

fn main() {
  println!("Hello, world!");
  let array = [1,2,3,4,5];
  println!("Test: {:?}", test_array(&array));
}

fn test_array<T>(array: &[T]) -> &[T] {
  return array
}
