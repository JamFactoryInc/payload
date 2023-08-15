use std::fmt::{Debug, Formatter};
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub(crate) struct Accumulator<T=u8>(Vec<T>);
impl<T> Accumulator<T> {

    pub(crate) fn vec_to_str(vec: Vec<u8>) -> String {
        unsafe {
            String::from_utf8_unchecked(vec)
        }
    }

    pub(crate) fn move_vec(&mut self) -> Vec<T> {
        let mut to_be_swapped = Vec::<T>::with_capacity(self.0.capacity());
        mem::swap(&mut self.0, &mut to_be_swapped);
        to_be_swapped
    }

    pub(crate) fn new() -> Accumulator {
        Accumulator(Vec::new())
    }
}
impl Accumulator<u8> {
    pub(crate) fn move_str(&mut self) -> String {
        let mut to_be_swapped = Vec::<u8>::with_capacity(self.0.capacity());
        mem::swap(&mut self.0, &mut to_be_swapped);
        Self::vec_to_str(to_be_swapped)
    }
}

impl<T> Deref for Accumulator<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Accumulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}