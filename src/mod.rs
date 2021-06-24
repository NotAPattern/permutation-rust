//use std::{cell::RefCell, collections::HashMap};
//#[derive(Debug)]

pub struct Permutation {
  permutation: Vec<usize>,
  length: usize,
  parity: bool,
  number_of_cycles: usize,
  pub inversion: usize,
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
    return_permutation.permutation = return_permutation.create_permutation(array);
    return_permutation.length = return_permutation.length();
    return_permutation.inversion = return_permutation.inversion();
    return_permutation.parity = false;
    return_permutation.number_of_cycles = return_permutation.cycles();

    return_permutation
  }

  fn create_permutation(&self, array: &[usize]) -> Vec<usize> {
    return array.to_vec();
  }

  fn length(&self) -> usize {
    return self.permutation.len();
  }

  fn inversion(&self) -> usize {
    let mut inversion = 0;
    for i in 0..(self.permutation.len() - 1) {
      for j in (i + 1)..(self.permutation.len()) {
        if self.permutation[i] > self.permutation[j] {
          inversion += 1;
        }
      }
    }
    return inversion;
  }
  /*
  fn parity(&self) -> bool {
    //
  }*/

  fn cycles(&self) -> usize {
    return 0;
  }
}
