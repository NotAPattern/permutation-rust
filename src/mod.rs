use std::{cell::RefCell, collections::HashMap};

mod permutation {

  struct SomeError;

  #[derive(Debug)]
  pub struct Permutation {
    permutation: Vec<u32>,
    length: usize,
    parity: u32,
    number_of_cycles: u32,
    inversion: u32,
  }

  impl Permutation {
    pub fn new<T>(&self, array: &[T]) -> Self {
      let return_permutation = Permutation {
        permutation: self.create_permutation(array),
        length: self.length(),
        parity: 0,
        number_of_cycles: 0,
        inversion: 0,
      };

      return return_permutation;
    }
    // pub fn new(&self, array: &[u32]) -> Self{
    //
    // }

    fn create_permutation<T>(&self, array: &[T]) -> Vec<u32> {
        let clone_permutation: Vec<u32>;
        for i in array {
            clone_permutation.push(*i.);
        }
      return clone_permutation; 
    }

    fn length(&self) -> usize {
      return self.permutation.len();
    }
  }
}
