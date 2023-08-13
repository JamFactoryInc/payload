use std::fmt::{Debug, Formatter};
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub(crate) struct Accumulator(String);
impl Accumulator {
    pub(crate) fn move_string(&mut self) -> String {
        let mut to_be_swapped = String::with_capacity(self.0.capacity());
        mem::swap(&mut self.0, &mut to_be_swapped);
        to_be_swapped
    }

    pub(crate) fn new() -> Accumulator {
        Accumulator(String::new())
    }
}

impl Deref for Accumulator {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Accumulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}