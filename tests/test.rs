#[path = "../src/lib.rs"]
pub mod lib;

use lib::permutation::Permutation;

#[cfg(test)]
mod tests {
  // This use to call private functions
  use super::*;
  #[test]
  fn create_permutation_with_vector() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let perm = Permutation::new(&vector);

    assert_eq!(vector, *perm.permutation());
  }

  #[test]
  fn create_permutation_with_array() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let perm = Permutation::new(&array);

    assert_eq!(array.to_vec(), *perm.permutation());
  }

  #[test]
  fn test_invesrion() {
    let test1 = 5;
    let test2 = 1;
    let test3 = 1;
    let test4 = 0;
    let perm1 = Permutation::new(&[1, 5, 3, 4, 2, 6]);
    let perm2 = Permutation::new(&[2, 1, 3, 4, 5]);
    let perm3 = Permutation::new(&[2, 1]);
    let perm4 = Permutation::new(&[1]);
    assert_eq!(test1, perm1.inversion());
    assert_eq!(test2, perm2.inversion());
    assert_eq!(test3, perm3.inversion());
    assert_eq!(test4, perm4.inversion());
  }

  #[test]
  fn test_parity() {
    let test1 = 1;
    let test2 = -1;
    let perm1 = Permutation::new(&[1, 5, 4, 3, 2, 6]);
    let perm2 = Permutation::new(&[2, 1, 3, 4, 5]);

    assert_eq!(test1, perm1.parity());
    assert_eq!(test2, perm2.parity());
  }

  #[test]
  fn test_cycles() {
    let test1 = 3;
    let test2 = 3;
    let test3 = 1;
    let perm1 = Permutation::new(&[2, 1, 4, 3, 5]);
    let perm2 = Permutation::new(&[4, 2, 7, 6, 5, 8, 1, 3]);
    let perm3 = Permutation::new(&[1]);
    assert_eq!(test1, perm1.cycles());
    assert_eq!(test2, perm2.cycles());
    assert_eq!(test3, perm3.cycles());
  }
}
