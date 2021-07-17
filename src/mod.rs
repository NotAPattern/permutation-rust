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
    let mut permutation_clone = self.permutation.clone();
    let mut number_of_cycles = 0;
    for index in 0..(permutation_clone.len() - 1) {
      let mut start = index;
      let mut current = index;
      let mut next = permutation_clone[index];
      if (next != 0) {
        number_of_cycles += 1;
      }
      while (start != next) {
        next = permutation_clone[next];
        permutation_clone[current] = 0;
        current = next;
      }
    }

    number_of_cycles
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

  pub fn length(&self) -> usize {
    self.length
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

  pub fn cycles(&self) -> usize {
    self.number_of_cycles
  }
}

impl Iterator for Permutation {
  type Item = Vec<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    // Narayana algorithm
    let mut min;
    let length = self.permutation.len();
    for i in (0..(length - 1)).rev() {
      if self.permutation[i] < self.permutation[i + 1] {
        min = i + 1;
        for j in (i + 1)..length {
          if (self.permutation[j] < self.permutation[min])
            && (self.permutation[j] > self.permutation[i])
          {
            min = j;
          }
        }
        self.permutation.swap(i, min);
        &self.permutation[(i + 1)..length].reverse();
        // TODO: Change `&mut Vec<usize>` or `Vec<usize>.clone()`?
        return Some(self.permutation.clone());
      }
    }
    None
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
    let perm1 = Permutation::new(&[2, 1, 4, 3, 5]);

    assert_eq!(test1, perm1.cycles());
  }
}
