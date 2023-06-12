use std::convert::AsRef;
use std::sync::Mutex;
use core::array::from_fn;
use std::ops::Index;

pub mod multidef_enum;
pub mod macros;
pub mod arena;
pub mod tree;
pub mod stack;


// struct ConstVec<'a, T> {
//     depth: usize,
//     filled: u8,
//     stack_array: [Option<T>; 8],
//     next : &'a Option<ConstVec<'a, T>>,
// }
// impl<'a, T> ConstVec<'a, T> {
//     #[inline]
//     const fn extend(mut self) {
//         let mut new_vec = ConstVec {
//             depth: self.depth + 1,
//             filled : 0,
//             stack_array : [None, None, None, None, None, None, None, None,],
//             next : &None
//         };
//         self.next = &Some(new_vec);
//     }
//     pub const fn new() -> ConstVec<'a, T> {
//         ConstVec::<'a, T> {
//             depth: 0,
//             filled: 0,
//             stack_array: [None, None, None, None, None, None, None, None,],
//             next: &None
//         }
//     }
//     pub const fn push(&self, val: T) -> usize {
//         if self.filled == 0xFF {
//             match self.next {
//                 Some(mut next) => {
//                     next.push(val)
//                 },
//                 None => {
//                     let mut new_vec = &ConstVec {
//                         depth: self.depth + 1,
//                         filled : 0,
//                         stack_array : [None, None, None, None, None, None, None, None,],
//                         next : &None
//                     };
//                     self.next = Some(new_vec);
//                     new_vec.push(val)
//                 }
//             }
//         } else {
//             const CURSOR : usize = 0b1000_0000;
//             let mut i = 0;
//             while i < 8 {
//                 let cursor = CURSOR >> i;
//                 if (self.filled as usize) < cursor {
//                     self.filled |= cursor as u8;
//                     self.stack_array[i] = Some(val);
//                     return self.depth * 8 + i;
//                 }
//                 i += 1;
//             }
//             panic!()
//         }
//     }
//     pub const fn set(&mut self, val: T, index: usize) {
//         if index < 8 {
//             self.stack_array[index] = Some(val);
//         } else {
//             match self.next {
//                 Some(mut next) => next.set(val, index - 8),
//                 None => {
//                     let mut new_vec = &ConstVec::new();
//                     self.next = Some(new_vec);
//                     new_vec.set(val, index - 8)
//                 }
//             }
//         }
//     }
//     pub const fn get(&mut self, index: usize) -> Option<&T> {
//         if index < 8 {
//             match self.stack_array.get(index) {
//                 Some(opt) => {
//                     Some(&opt.unwrap())
//                 },
//                 None => None
//             }
//         } else {
//             match self.next {
//                 Some(mut next) => next.get(index - 8),
//                 None => None
//             }
//         }
//     }
// }

//const STRING_REGISTRY: Mutex<&ConstVec<& str>> = Mutex::new(&ConstVec::new());
// const REGISTERED_COUNT: Mutex<usize> = Mutex::new(0);
// pub struct RegisteredString {}
// impl RegisteredString {
//     pub fn new(str : &str) -> usize {
//         let mut vec = *STRING_REGISTRY.lock().unwrap();
//         vec.push(str);
//         let mut val = *REGISTERED_COUNT.lock().unwrap();
//         val += 1;
//         val.clone()
//     }
//     pub fn get(id: &usize) -> Option<&str> {
//         match STRING_REGISTRY.lock().unwrap().get(*id) {
//             Some(s) => Some(*s),
//             None => None
//         }
//     }
// }
