// Let's write a vtable! We'll use it to pass around a trait objeect implementing Display.
// https://fasterthanli.me/articles/the-curse-of-strong-typing#dynamically-sized-types

use std::{
  fmt::{self, Display},
  intrinsics::transmute,
};

/// This is our Display trait object
struct BoxedDisplay {
  /// A raw pointer to some value, which lives on the heap. This is the object that we claim will
  /// implement Display.
  data:   *mut (),
  /// This is a reference the table of methods that Display guarantees
  vtable: &'static DisplayVtable<()>,
}

// NOT TODAY
// impl Display for BoxedDisplay{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

/// SUFFER WORM! Implement the mothods for Display. I guess that includes drop.
#[repr(C)]
struct DisplayVtable<T> {
  /// We need this to be an pointer to a function satisfying the fmt contract.
  fmt:  unsafe fn(*mut T, &mut fmt::Formatter<'_>) -> fmt::Result,
  /// another pointer to the method that de-allocates... Hmm. self, I guess.
  drop: unsafe fn(*mut T),
}

impl<T: Display> DisplayVtable<T> {
  // build yourself a vtable, implementing display
  fn new() -> &'static Self {
    // forward to T's display impl, deferencing whatever `this` is
    unsafe fn fmt<T: Display>(this: *mut T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      // this.fmt(f) // this also works: don't need to manually coerce the reference
      // wait, I take it back, it definitely doesn't work, if I type this correctly
      (*this).fmt(f) // given code
    }
    /// another pointer tot he method that de-allocates... Hmm. self, I guess.
    unsafe fn drop<T>(this: *mut T) {
      // take ownership of this, and fall out of scopee (dropping whatever this is)
      //  Box::from_raw(this); // this doesn't use the result (given code)
      let _ = Box::from_raw(this);
    }
    // since both of these are regular functions (not closures), they end up in the executable and
    // live for 'static
    &Self { fmt, drop }
  }
}

// now we can construct BoxedDisplay
impl BoxedDisplay {
  // 'static means that T is definitely owned.
  fn new<T: Display + 'static>(t: T) -> Self {
    //  What do you do to make std::alloc::Global opaque?
    // Self { data: Box::into_raw(Box::from(t)) , vtable: () }
    Self {
      // lol, this of course. What's this?👇️
      data:   Box::into_raw(Box::from(t)) as _,
      // we're going to transmute our vtable into the vtable that we actually wanted? Ok, why not
      vtable: unsafe { transmute(DisplayVtable::<T>::new()) },
      // vtable: DisplayVtable::<T>::new(), // expected <()> got <T>
      // vtable: DisplayVtable::<()>::new(), // () does not satisfy Display
    }
  }
}

impl Display for BoxedDisplay {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // call the self.vtable fmt method on self.data, with the given formatter
    unsafe { (self.vtable.fmt)(self.data, f) }
  }
}

impl Drop for BoxedDisplay {
  fn drop(&mut self) { unsafe { (self.vtable.drop)(self.data) } }
}

// finally, let's use our Display vtable. all of these should work, since they all implement Display
fn get_display_trait_object() -> BoxedDisplay {
  // BoxedDisplay::new("c")
  // BoxedDisplay::new(1)
  // BoxedDisplay::new("c".to_string())
  BoxedDisplay::new(true)
}
