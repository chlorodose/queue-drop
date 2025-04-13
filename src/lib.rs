#![feature(decl_macro, f16, f128)]
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use core::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

mod primitive_impl;
mod std_impl;

pub use queue_drop_macro::{QueueDrop, side_drop};

/// Run the drop code for struct itself and push all sub-element into the queue
pub trait QueueDrop {
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>);
}

/// Custom code within the destructor.
/// We need this trait to bypass the limit where Rust doesn't allow us to invoke Drop::drop
pub trait SideDrop {
    unsafe fn drop(&mut self);
}

pub struct QueueDropBox<T: QueueDrop> {
    inner: ManuallyDrop<T>,
}
impl<T: QueueDrop> Deref for QueueDropBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
impl<T: QueueDrop> DerefMut for QueueDropBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}
impl<T: QueueDrop> QueueDropBox<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: ManuallyDrop::new(inner),
        }
    }
}
impl<T: QueueDrop> Drop for QueueDropBox<T> {
    fn drop(&mut self) {
        let mut queue = Vec::new();
        queue.push(self.inner.deref_mut() as &mut dyn QueueDrop);
        while let Some(item) = queue.pop() {
            unsafe { item.queue_drop(&mut queue) };
        }
    }
}
