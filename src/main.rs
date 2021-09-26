pub mod r#mod;

use r#mod::Permutation;

fn main() {
  println!("Hello, world!");
  let vector2 = vec![4, 2, 7, 6, 5, 8, 1, 3];
  //let mut iter = vector2.iter();
  let mut perm = Permutation::new(&vector2);
  println!("{:?}", perm.permutation());
  while let Some(item) = perm.next() {
    println!("{:?}", item);
  }
}
