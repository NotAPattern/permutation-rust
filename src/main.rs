pub mod r#mod;

use r#mod::Permutation;

fn main() {
  println!("Hello, world!");
  let array = [1, 5, 3, 4, 2, 6];
  let mut vector = vec![2, 1, 3, 4, 5];
  let vector2 = vec![1, 2, 3];
  let perm1 = Permutation::new(&array);
  let perm2 = Permutation::new(&vector);
  let mut perm3 = Permutation::new(&vector2);
  println!("{}", perm1.inversion());
  println!("{}", perm2.inversion());
  println!("{}", perm2.parity());
  println!("{:?}", perm3.permutation());
  println!("{}", perm3.permutation()[0]);
  println!("{:?}", perm3.next());
  println!("{:?}", perm3.next());
  println!("{:?}", perm3.next());
  println!("{:?}", perm3.next());
  println!("{}", perm3.permutation().len());
  for i in (0..perm3.permutation().len() - 1).rev() {
    println!("{}", i);
  }
}
