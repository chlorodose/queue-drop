use alloc::vec::Vec;

use crate::{QueueDrop, SideDrop};

macro impl_empty($i: ident) {
    impl SideDrop for $i {
        unsafe fn drop(&mut self) {}
    }
    impl QueueDrop for $i {
        unsafe fn queue_drop<'a>(self: &'a mut Self, _queue: &mut Vec<&'a mut dyn QueueDrop>) {}
    }
}
impl_empty!(bool);
impl_empty!(char);
impl_empty!(f16);
impl_empty!(f32);
impl_empty!(f64);
impl_empty!(f128);
impl_empty!(i8);
impl_empty!(i16);
impl_empty!(i32);
impl_empty!(i64);
impl_empty!(i128);
impl_empty!(isize);
impl_empty!(u8);
impl_empty!(u16);
impl_empty!(u32);
impl_empty!(u64);
impl_empty!(u128);
impl_empty!(usize);
impl_empty!(str);

impl<T> SideDrop for *const T {
    unsafe fn drop(&mut self) {}
}
impl<T> QueueDrop for *const T {
    unsafe fn queue_drop<'a>(self: &'a mut Self, _queue: &mut Vec<&'a mut dyn QueueDrop>) {}
}
impl<T> SideDrop for *mut T {
    unsafe fn drop(&mut self) {}
}
impl<T> QueueDrop for *mut T {
    unsafe fn queue_drop<'a>(self: &'a mut Self, _queue: &mut Vec<&'a mut dyn QueueDrop>) {}
}
impl<'a, T> SideDrop for &'a T {
    unsafe fn drop(&mut self) {}
}
impl<'a, T> QueueDrop for &'a T {
    unsafe fn queue_drop<'b>(self: &'b mut Self, _queue: &mut Vec<&'b mut dyn QueueDrop>) {}
}
impl<'a, T> SideDrop for &'a mut T {
    unsafe fn drop(&mut self) {}
}
impl<'a, T> QueueDrop for &'a mut T {
    unsafe fn queue_drop<'b>(self: &'b mut Self, _queue: &mut Vec<&'b mut dyn QueueDrop>) {}
}

impl<T: SideDrop, const N: usize> SideDrop for [T; N] {
    unsafe fn drop(&mut self) {}
}
impl<T: QueueDrop, const N: usize> QueueDrop for [T; N] {
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        self.iter_mut()
            .rev()
            .for_each(|r| queue.push(r as &mut dyn QueueDrop));
    }
}
impl<T: SideDrop> SideDrop for [T] {
    unsafe fn drop(&mut self) {}
}
impl<T: QueueDrop> QueueDrop for [T] {
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        self.iter_mut()
            .rev()
            .for_each(|r| queue.push(r as &mut dyn QueueDrop));
    }
}
impl<A: SideDrop, B: SideDrop> SideDrop for (A, B) {
    unsafe fn drop(&mut self) {}
}
impl<A: SideDrop, B: SideDrop, C: SideDrop> SideDrop for (A, B, C) {
    unsafe fn drop(&mut self) {}
}
impl<A: SideDrop, B: SideDrop, C: SideDrop, D: SideDrop> SideDrop for (A, B, C, D) {
    unsafe fn drop(&mut self) {}
}
impl<A: SideDrop, B: SideDrop, C: SideDrop, D: SideDrop, E: SideDrop> SideDrop for (A, B, C, D, E) {
    unsafe fn drop(&mut self) {}
}
impl<A: SideDrop, B: SideDrop, C: SideDrop, D: SideDrop, E: SideDrop, F: SideDrop> SideDrop
    for (A, B, C, D, E, F)
{
    unsafe fn drop(&mut self) {}
}
impl<A: SideDrop, B: SideDrop, C: SideDrop, D: SideDrop, E: SideDrop, F: SideDrop, G: SideDrop>
    SideDrop for (A, B, C, D, E, F, G)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
    K: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J, K)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
    K: SideDrop,
    L: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J, K, L)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
    K: SideDrop,
    L: SideDrop,
    M: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
    K: SideDrop,
    L: SideDrop,
    M: SideDrop,
    N: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
    K: SideDrop,
    L: SideDrop,
    M: SideDrop,
    N: SideDrop,
    O: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
{
    unsafe fn drop(&mut self) {}
}
impl<
    A: SideDrop,
    B: SideDrop,
    C: SideDrop,
    D: SideDrop,
    E: SideDrop,
    F: SideDrop,
    G: SideDrop,
    H: SideDrop,
    I: SideDrop,
    J: SideDrop,
    K: SideDrop,
    L: SideDrop,
    M: SideDrop,
    N: SideDrop,
    O: SideDrop,
    P: SideDrop,
> SideDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
{
    unsafe fn drop(&mut self) {}
}
impl<A: QueueDrop, B: QueueDrop> QueueDrop for (A, B) {
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<A: QueueDrop, B: QueueDrop, C: QueueDrop> QueueDrop for (A, B, C) {
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<A: QueueDrop, B: QueueDrop, C: QueueDrop, D: QueueDrop> QueueDrop for (A, B, C, D) {
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<A: QueueDrop, B: QueueDrop, C: QueueDrop, D: QueueDrop, E: QueueDrop> QueueDrop
    for (A, B, C, D, E)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<A: QueueDrop, B: QueueDrop, C: QueueDrop, D: QueueDrop, E: QueueDrop, F: QueueDrop> QueueDrop
    for (A, B, C, D, E, F)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
    K: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J, K)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.10 as &mut dyn QueueDrop);
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
    K: QueueDrop,
    L: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J, K, L)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.11 as &mut dyn QueueDrop);
        queue.push(&mut self.10 as &mut dyn QueueDrop);
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
    K: QueueDrop,
    L: QueueDrop,
    M: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.12 as &mut dyn QueueDrop);
        queue.push(&mut self.11 as &mut dyn QueueDrop);
        queue.push(&mut self.10 as &mut dyn QueueDrop);
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
    K: QueueDrop,
    L: QueueDrop,
    M: QueueDrop,
    N: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.13 as &mut dyn QueueDrop);
        queue.push(&mut self.12 as &mut dyn QueueDrop);
        queue.push(&mut self.11 as &mut dyn QueueDrop);
        queue.push(&mut self.10 as &mut dyn QueueDrop);
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
    K: QueueDrop,
    L: QueueDrop,
    M: QueueDrop,
    N: QueueDrop,
    O: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.14 as &mut dyn QueueDrop);
        queue.push(&mut self.13 as &mut dyn QueueDrop);
        queue.push(&mut self.12 as &mut dyn QueueDrop);
        queue.push(&mut self.11 as &mut dyn QueueDrop);
        queue.push(&mut self.10 as &mut dyn QueueDrop);
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
impl<
    A: QueueDrop,
    B: QueueDrop,
    C: QueueDrop,
    D: QueueDrop,
    E: QueueDrop,
    F: QueueDrop,
    G: QueueDrop,
    H: QueueDrop,
    I: QueueDrop,
    J: QueueDrop,
    K: QueueDrop,
    L: QueueDrop,
    M: QueueDrop,
    N: QueueDrop,
    O: QueueDrop,
    P: QueueDrop,
> QueueDrop for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
{
    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
        queue.push(&mut self.15 as &mut dyn QueueDrop);
        queue.push(&mut self.14 as &mut dyn QueueDrop);
        queue.push(&mut self.13 as &mut dyn QueueDrop);
        queue.push(&mut self.12 as &mut dyn QueueDrop);
        queue.push(&mut self.11 as &mut dyn QueueDrop);
        queue.push(&mut self.10 as &mut dyn QueueDrop);
        queue.push(&mut self.9 as &mut dyn QueueDrop);
        queue.push(&mut self.8 as &mut dyn QueueDrop);
        queue.push(&mut self.7 as &mut dyn QueueDrop);
        queue.push(&mut self.6 as &mut dyn QueueDrop);
        queue.push(&mut self.5 as &mut dyn QueueDrop);
        queue.push(&mut self.4 as &mut dyn QueueDrop);
        queue.push(&mut self.3 as &mut dyn QueueDrop);
        queue.push(&mut self.2 as &mut dyn QueueDrop);
        queue.push(&mut self.1 as &mut dyn QueueDrop);
        queue.push(&mut self.0 as &mut dyn QueueDrop);
    }
}
