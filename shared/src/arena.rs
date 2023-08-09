use std::alloc::{alloc, Allocator, AllocError, dealloc, Global, handle_alloc_error, Layout, LayoutError};
use std::error::Error;
use std::intrinsics::{ceilf32, log2f32};
use std::marker::{PhantomData, PhantomPinned};
use std::mem::size_of;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

pub struct ArenaId {
    block_index: u32,
    mem_index: u32,
}

pub struct Arena<'a, T: Sized> {
    _phantom: PhantomData<&'a T>,
    block: NonNull<[u8]>,
    blocks: Vec<NonNull<[u8]>>,
    layout: Layout
}
impl<'a, T: Sized> Arena<'a, T> {
    pub fn new() -> Result<Arena<'a, T>, LayoutError> {
        let layout = Layout::from_size_align(
            size_of::<T>(),
            8
        ).unwrap();
        let block = Self::alloc(&layout)?;
        Ok(Arena {
            _phantom: Default::default(),
            block,
            blocks: vec![block],
            layout,
        })
    }

    fn alloc(layout: &Layout) -> Result<NonNull<[u8]>, Box<dyn Error>> {
        unsafe {
            Ok(Global::allocate(&Global, layout.repeat_packed(128)?)?)
        }
    }

    fn save(&mut self, ) {
        //self.blocks.push()
    }
}

impl<'a, T> Index<ArenaId> for Arena<'a, T> {
    type Output = ();

    fn index(&self, index: ArenaId) -> &'a Self::Output {
        todo!()
    }
}

impl<'a, T> IndexMut<ArenaId> for Arena<'a, T> {
    fn index_mut(&mut self, index: ArenaId) -> &'a mut Self::Output {
        todo!()
    }
}

impl<'a, T> Drop for Arena<'a, T> {
    fn drop(&mut self) {
        unsafe {
            for ptr in self.blocks.iter() {
                dealloc(*ptr, self.layout.repeat_packed(128).unwrap())
            }
        }
    }
}