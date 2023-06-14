use std::array::from_fn;
use std::ffi::c_void;
use std::future::Future;
use std::mem;
use std::mem::size_of;
use std::sync::{Arc, Condvar, Mutex};
use rand::seq::index::sample_weighted;

struct Stack<T> {
    top: Option<T>,
    cursor : u8,
    dead_cache_loaded : bool,
    lock : (Mutex<bool>, Condvar),
    cache: *mut [Option<T>; 4],
    dead_cache: *mut [Option<T>; 4],
    reserve: Vec<T>,
}

pub struct StackResidence<T> {
    cache: [Option<T>; 4],
    dead_cache: [Option<T>; 4],
}
impl<T> StackResidence<T> {
    fn get_stack(&mut self) -> Stack<T> {
        Stack {
            top: None,
            cursor: 0,
            dead_cache_loaded: false,
            lock : (Mutex::new(false), Condvar::new()),
            cache: &mut self.cache as *mut [Option<T>; 4],
            dead_cache: &mut self.dead_cache as *mut [Option<T>; 4],
            reserve: Vec::new(),
        }
    }
}

impl<'a, T> Stack<T> {
    fn reside() -> StackResidence<T> {
        StackResidence::<T> {
            cache: [None, None, None, None],
            dead_cache: [None, None, None, None],
        }
    }

    fn wait_for_unlock(&mut self) {
        let mut started = self.lock.0.lock().unwrap();
        while !*started {
            started = self.lock.1.wait(started).unwrap()
        }
    }

    fn locked<C>(&mut self, action: C) where C: Fn(&mut Self) -> () {
        *self.lock.0.lock().unwrap() = true;
        action(self);
        *self.lock.0.lock().unwrap() = false;
        self.lock.1.notify_one()
    }

    async fn load_dead(&mut self) {
        self.locked(|s| {
            unsafe {
                mem::swap(s.dead_cache.as_mut().unwrap(), s.cache.as_mut().unwrap());
            }
            let mut drain = match s.reserve.len() {
                len @ 0..=3 => {
                    s.reserve.drain(0..len)
                }
                _ => s.reserve.drain(0..4)
            };
            unsafe {
                *s.dead_cache = from_fn(|_| drain.next())
            }
            s.dead_cache_loaded = true;
        });
    }

    async fn unload_dead(&mut self) {
        self.locked(move |s| {
            unsafe {
                let new_len = s.reserve.len() + 4;
                s.reserve.reserve(4);
                s.reserve.set_len(new_len);

                let mut swapped_dead_cache = [None, None, None, None];
                // swap this ^^ array with the dead cache
                mem::swap(&mut swapped_dead_cache, s.dead_cache.as_mut().unwrap());
                // swap the new value of the dead cache with the old cache
                mem::swap(s.dead_cache.as_mut().unwrap(), s.cache.as_mut().unwrap());

                let src = s.reserve.as_mut_ptr().offset(s.reserve.len() as isize);

                std::ptr::copy_nonoverlapping( &swapped_dead_cache.map(|val| val.unwrap())[0], src, 4)
            }
        });
    }

    fn pop(&mut self) -> Option<T> {
        let ret_val = Some(self.top.take());
        match self.cursor {
            0..=1 => {
                unsafe {
                    self.top = (*self.cache)[3].take()
                }
                self.cursor += 1;
            },
            _ => {
                self.load_dead()
            }
        }
        if (self.cursor == 2) {


        }
        todo!()
    }
}

#[test]
fn test() {

}