pub mod r#mod;

use r#mod::Permutation;

fn main() {
  println!("Hello, world!");
  let array = [1, 5, 3, 4, 2, 6];
  let vector = vec![2, 1, 3, 4, 5];
  let perm1 = Permutation::new(&array);
  let perm2 = Permutation::new(&vector);
  println!("{}", perm1.inversion());
  println!("{}", perm2.inversion());
  println!("{}", perm2.parity());
}
