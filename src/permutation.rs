use std::collections::HashMap;
use std::cell::RefCell;

mod permutation {

struct SomeError;

#[derive(Debug)]
pub struct Permutation {
  permutation: Vec<u32>,
  length: usize,
  parity: u32,
  number_of_cycles: u32,
  inversion: u32
}

impl Permutation {
  pub fn new(&self, array: std::cell::RefCell<Vec<u32>>) -> Self {
    let return_permutation = Permutation {
        permutation: self.create_permutation(array),
        length: self.length(), 
        parity: 0,
        number_of_cycles: 0,
        inversion: 0,
    };

    return return_permutation
  }

  fn create_permutation(&self, array: std::cell::RefCell<Vec<u32>>) -> Vec<u32> {
   return array.borrow().to_vec()
  }

  fn length(&self) -> usize {
    return self.permutation.len()
  } 
}

}
