use std::collections::HashMap;

mod permutation {

#[derive(Debug)]
pub struct Permutation {
  permutation: Vec<usize>,
  length: usize,
  parity: usize,
  number_of_cycles: usize,
  inversion: usize
}

impl Permutation {
  pub fn new<T>(&self, array: &mut[usize]) -> Permutation {
    let return_permutation = Permutation{
        permutation: self.create_permutation(array),
        length: self.length(), 
        parity: 0,
        number_of_cycles: 0,
        inversion: 0,
    };

    return return_permutation
  }

  fn create_permutation<'a>(&self, array: &mut [usize]) -> Vec<usize> {
   return array.to_vec()
  }

  fn length(&self) -> usize {
    return self.permutation.len()
  }
}

}
