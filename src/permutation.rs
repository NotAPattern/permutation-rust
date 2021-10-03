/// Structure of permutation
pub struct Permutation {
  // Maybe in furure `T` or `Vec<T>`
  /// All permutation store in Vec<usize>
  permutation: Vec<usize>,
  // true – even, false – odd
  /// Parity of permutation. *True* if *even*, *false* if odd
  parity: bool,
  /// Number of cycles in permutation
  number_of_cycles: usize,
  /// Number of inversion in permutation
  inversion: usize,
}

impl Permutation {
  fn get_create_permutation(&self, array: &[usize]) -> Vec<usize> {
    array.to_vec()
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

  fn get_parity(&mut self) -> bool {
    self.inversion = self.get_inversion();
    return if self.inversion % 2 == 0 { true } else { false };
  }

  fn get_cycles(&self) -> usize {
    let mut permutation_clone = self.permutation.clone();
    let mut number_of_cycles = 0;
    for index in 0..permutation_clone.len() {
      let mut current_index = index;
      let mut next = permutation_clone[index];
      if permutation_clone[index] != 0 {
        number_of_cycles += 1;
      }
      while next != 0 {
        permutation_clone[current_index] = 0;
        current_index = next - 1;
        if permutation_clone[next - 1] != 0 {
          next = permutation_clone[next - 1];
        } else {
          next = 0;
        }
      }
    }
    number_of_cycles
  }
}

impl Permutation {
  // TODO: maybe `Option<Self>` ?
  /// Create permutation based on slice of `usize`.
  ///
  /// Start indexing with 1.
  ///
  /// Example:
  ///   * [1, 2, 3, 4, 5] is **correct** vector,
  ///   * [0, 1, 2] is **not correct** vector.
  /// # Example
  ///   ```
  ///   use permutation::permutation::Permutation;
  ///
  ///   let vector = vec![1, 2, 3, 4, 5];
  ///   let permutation = Permutation::new(&vector);
  ///
  ///   assert_eq!(vector, *permutation.permutation());
  ///   ```
  pub fn new(array: &[usize]) -> Self {
    /*if array.len() == 0 {
      return None;
    }*/
    let mut return_permutation = Permutation {
      permutation: vec![0],
      parity: false,
      number_of_cycles: 0,
      inversion: 0,
    };
    return_permutation.permutation = return_permutation.get_create_permutation(array);
    return_permutation.inversion = return_permutation.get_inversion();
    return_permutation.parity = return_permutation.get_parity();
    return_permutation.number_of_cycles = return_permutation.get_cycles();

    return_permutation
  }

  /// Return length of permutation.
  ///
  /// # Exapmle
  /// ```
  ///   use permutation::permutation::Permutation;
  ///
  ///   let vector = vec![1, 2, 3, 4, 5];
  ///   let permutation = Permutation::new(&vector);
  ///
  ///   assert_eq!(5, permutation.length());
  ///
  /// ```
  pub fn length(&self) -> usize {
    self.permutation.len()
  }

  /// Return permutation in `Vec<usize>`.
  ///
  /// # Example
  /// ```
  ///   use permutation::permutation::Permutation;
  ///
  ///   let vector = vec![1, 2, 3, 4, 5];
  ///   let permutation = Permutation::new(&vector);
  ///
  ///   assert_eq!(vector, *permutation.permutation());
  ///
  /// ```
  pub fn permutation(&self) -> &Vec<usize> {
    &self.permutation
  }

  /// Return numer of [inversions](https://en.wikipedia.org/wiki/Permutation#Inversions).
  /// Example:
  ///   * [1, 2, 4, 3] have 1 inversion.
  ///
  /// # Example
  /// ```
  ///   use permutation::permutation::Permutation;
  ///
  ///   let vector = vec![1, 2, 3, 5, 4];
  ///   let permutation = Permutation::new(&vector);
  ///   
  ///   assert_eq!(1, permutation.inversion());
  /// ```
  pub fn inversion(&self) -> usize {
    self.inversion
  }

  /// Return [parity](https://en.wikipedia.org/wiki/Permutation#Parity_of_a_permutation) of permutation.
  /// Return:
  ///   * 1 – permutation if even,
  ///   * -1 – permutation if odd.
  ///
  /// # Example
  /// ```
  ///   use permutation::permutation::Permutation;
  ///
  ///   let vector = vec![1, 2, 3, 4, 5];
  ///   let permutation = Permutation::new(&vector);
  ///   
  ///   assert_eq!(1, permutation.parity());
  pub fn parity(&self) -> isize {
    return if self.parity { 1 } else { -1 };
  }

  /// Return numer of [cycles](https://en.wikipedia.org/wiki/Permutation#Canonical_cycle_notation).
  /// Example:
  ///
  ///   * [2, 1, 4, 3] have 2 cycles.
  ///
  /// # Example
  /// ```
  ///   use permutation::permutation::Permutation;
  ///
  ///   let vector = vec![2, 1, 4, 3, 5];
  ///   let permutation = Permutation::new(&vector);
  ///   
  ///   assert_eq!(3, permutation.cycles());
  /// ```
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
        self.permutation[(i + 1)..length].reverse();
        // TODO: Change `&mut Vec<usize>` or `Vec<usize>.clone()`?
        self.inversion = self.get_inversion();
        self.parity = self.get_parity();
        self.number_of_cycles = self.get_cycles();
        return Some(self.permutation.clone());
      }
    }
    self.inversion = self.get_inversion();
    self.parity = self.get_parity();
    self.number_of_cycles = self.get_cycles();
    None
  }
}
