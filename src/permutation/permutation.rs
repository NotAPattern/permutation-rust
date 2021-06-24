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
  pub fn new<T>(&self, array: &[usize]) -> Self {
    let return_permutation = Permutation {
      permutation: self.create_permutation(array),
      length: self.permutation.len(),
      inversion: self.inversion(),
      parity: false,
      number_of_cycles: self.cycles(),
    };

    return return_permutation;
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
      for j in (i + 1)..(self.permutation.len() - 1) {
        if (self.permutation[i] < self.permutation[j]) {
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
