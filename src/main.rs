pub mod r#mod;

use r#mod::Permutation;

fn main() {
  println!("Hello, world!");
  let mut vector2 = vec![1, 2, 3, 4];
  let mut iter = vector2.iter();
  let mut perm = Permutation::new(&vector2);
  println!("{:?}", perm.permutation());
  while let Some(item) = perm.next() {
    println!("{:?}", item);
  }
}
