use std::intrinsics::transmute_unchecked;
use std::mem::transmute;
use std::simd::{u32x32, u8x32};

const POLY: u8x32 = u8x32::from_array([31; 32]);
const POLY_POW: u32x32 = u32x32::from_array([31; 32]);

pub struct Scope {
    path: Vec<[u8; 32]>,
    id: u64
}
impl Scope {
    pub(crate) fn hash_ststr(path: &'static str) {
        debug_assert!(path.as_bytes().len() <= 32);
    }

    pub(crate) fn hash_str(path: String) {
        debug_assert!(path.as_bytes().len() <= 32);
        unsafe {
            let src: u8x32 = transmute_unchecked([0u8;32].copy_from_slice(path.as_bytes()));


        }
    }

    pub(crate) fn add_hash() {

    }

    pub(crate) fn rem_hash() {

    }

    pub fn add(&mut self, path: &'static str) {
        debug_assert!(path.len() <= 32);
        unsafe {
            let src: u8x32 = transmute_unchecked([0u8;32].copy_from_slice(path.as_bytes()));
        }
    }
}

pub enum ScopedElement<T> {
    StartScope,
    EndScope,
    Element(T)
}