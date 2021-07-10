//use std::{cell::RefCell, collections::HashMap};
//#[derive(Debug)]

pub struct Permutation {
  // Maybe in furure `T` or `Vec<T>`
  permutation: Vec<usize>,
  length: usize,
  // true – even, false – odd
  parity: bool,
  number_of_cycles: usize,
  inversion: usize,
}

impl Permutation {
  fn get_create_permutation(&self, array: &[usize]) -> Vec<usize> {
    array.to_vec()
  }

  fn get_length(&self) -> usize {
    self.permutation.len()
  }

  fn get_inversion(&self) -> usize {
    let mut inversion = 0;
    for i in 0..(self.permutation.len() - 1) {
      for j in (i + 1)..(self.permutation.len()) {
        if self.permutation[i] > self.permutation[j] {
          inversion += 1;
        }
      }
    }

    inversion
  }

  fn get_parity(&self) -> bool {
    return if self.inversion % 2 == 0 { true } else { false };
  }

  fn get_cycles(&self) -> usize {
    0
  }
}

impl Permutation {
  pub fn new(array: &[usize]) -> Self {
    let mut return_permutation = Permutation {
      permutation: vec![0],
      length: 0,
      parity: false,
      number_of_cycles: 0,
      inversion: 0,
    };
    return_permutation.permutation = return_permutation.get_create_permutation(array);
    return_permutation.length = return_permutation.get_length();
    return_permutation.inversion = return_permutation.get_inversion();
    return_permutation.parity = return_permutation.get_parity();
    return_permutation.number_of_cycles = return_permutation.get_cycles();

    return_permutation
  }

  pub fn permutation(&self) -> &Vec<usize> {
    &self.permutation
  }

  pub fn inversion(&self) -> usize {
    self.inversion
  }

  pub fn parity(&self) -> isize {
    return if self.parity { 1 } else { -1 };
  }
}

#[cfg(test)]
mod tests {
  //pub mod Permutation;
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
}
