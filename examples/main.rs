#[path = "../src/lib.rs"]
pub mod lib;

use lib::permutation::Permutation;

fn main() {
  println!("Hello, world!");
  let vector2 = vec![2, 1];
  //let mut iter = vector2.iter();
  let mut perm = Permutation::new(&vector2);
  println!("{:?}", perm.permutation());
  while let Some(item) = perm.next() {
    println!("{:?}", item);
  }
}
