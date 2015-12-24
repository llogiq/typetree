pub use typenum::const::*;
use std::marker::PhantomData;

pub struct Octet<T1, T2, T3, T4, T5, T6, T7, T8> {
    __data: PhantomData<(T1, T2, T3, T4, T5, T6, T7, T8)>,
}

pub trait Tree {}

impl Tree for Octet<T1, T2, T3, T4, T5, T6, T7, T8> {}

/// Get a type out of our tree
pub trait Get<P: UInt> {
    type Output;
}

impl Get<U0> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T1; }
impl Get<U1> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T2; }
impl Get<U2> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T3; }
impl Get<U3> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T4; }
impl Get<U4> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T5; }
impl Get<U5> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T6; }
impl Get<U6> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T7; }
impl Get<U7> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { type Output = T8; }

/// Put a type value into the tree at some position
pub trait Put<P: UInt, T> {
    type Output;
}

impl Put<U0, T> for Octet<T1, T2, T3, T4, T5, T6, T7, T8> { 
    type Output = Octet<T, T2, T3, T4, T5, T6, T7, T8>;
}
