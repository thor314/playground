// try implementing Map for Iterator<Item = T>

// equip mymap with a lifetime that lives as long as the iterator.
struct MyMap<'a, I, O, F>
where F: FnOnce(I) -> O {
  // it: Iterator<Self::Item = T>
  // must be mutable so we can call next on it
  pub it: &'a mut dyn Iterator<Item = O>,
  pub f:  F,
}

impl<'a, I, O, F> MyMap<'a, I, O, F> {
  pub fn new(it: &mut dyn Iterator<Item = I>, f: F) -> Self {
    {
      Self { it, f }
    }
  }
}

trait MyTrait {
  type Item<O>;
  // fn my_map<T, S>(&mut self) -> MyMap<S>;
  // fn my_map< S>(&mut self) -> MyMap<S>;
  fn my_map<I, O, F>(&mut self, f: F) -> MyMap<O>
  where F: FnOnce(I) -> O;
}

impl<'a, I, O, F> MyTrait for dyn Iterator<Item = I> {
  fn my_map(&mut self, f: dyn FnOnce(I) -> O) -> MyMap<'a, I , O ,F> {
    while let Some(v) = self.next() {
      MyMap::new(, f)
    }
  }
}

// Iterator combinator
impl<'a, T> Iterator for MyMap<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> { self.it.next() }
}

fn tryit() { let _ = (1..2).into_iter().my_map(|n| n).collect(); }
