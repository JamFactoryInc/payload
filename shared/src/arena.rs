use std::marker::PhantomData;

pub struct Arena<'a, T> {
    _phantom: PhantomData<T>

}