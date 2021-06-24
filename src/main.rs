mod permutation;

fn main() {
  println!("Hello, world!");
  let array = [1, 2, 3, 4, 5];
  let vector = vec![1, 2, 3, 4, 5];
  let mut perm = permutation::permutation::Permutation::new(array);
  println!("{}", perm.inversion);
}
