use std::alloc::Layout;
use std::marker::PhantomData;

trait Arena<T> {
    fn add(&mut self, val: T) -> usize;
    fn rem(&mut self, index: usize);
    fn get(&self, index: usize) -> T;
    fn clear(&mut self) -> T;
}

// struct VecArena<T> {
//     buf : Vec<T>,
//     empty : std::collections::
// }
// impl<T> Arena<T> for VecArena<T> {
//     fn add(&mut self, val: T) -> usize {
//         self.buf.push(val)
//     }
//
//     fn rem(&mut selfmindex: usize) {
//         todo!()
//     }
//
//     fn get(&self, index: usize) -> T {
//         todo!()
//     }
//
//     fn clear(&mut self) -> T {
//         todo!()
//     }
// }
