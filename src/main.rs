mod r#mod;

use std::cell::RefCell;

fn main() {
  println!("Hello, world!");
  let array = [1, 2, 3, 4, 5];
  let vector = vec![1,2,3,4,5];
  println!("Test: {:?}", test_array(&array));
  println!("Test vec: {:?}", test_array(&vector));
  print_if_string(&0);
  print_if_string(&"cookie monster".to_string());
}

fn test_array<T>(array: &[T]) -> &[T] {
  return array;
}

use std::any::Any;

fn print_if_string(s: &dyn Any) {
  //let string = s.downcast_ref::<String>();
  //println!("{:?}", string);
  if let Some(string) = s.downcast_ref::<String>() {
    println!("It's a string({}): '{}'", string.len(), string);
  } else {
    println!("Not a string...");
  }
}
