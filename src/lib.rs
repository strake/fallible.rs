#![no_std]

extern crate void;

pub mod iter;

use void::Void;

pub trait TryClone: Sized {
    type Error;
    fn try_clone(&self) -> Result<Self, Self::Error>;
    fn try_clone_from(&mut self, other: &Self) -> Result<(), Self::Error> {
        other.try_clone().map(|new| *self = new)
    }
}

impl<T: Clone> TryClone for T {
    type Error = Void;

    #[inline(always)]
    fn try_clone(&self) -> Result<Self, Void> { Ok(self.clone()) }

    #[inline(always)]
    fn try_clone_from(&mut self, other: &Self) -> Result<(), Void> {
        Ok(self.clone_from(other))
    }
}

pub trait TryFrom<A>: Sized {
    type Error;
    fn try_from(A) -> Result<Self, Self::Error>;
}

impl<A, T: From<A>> TryFrom<A> for T {
    type Error = Void;

    #[inline(always)]
    fn try_from(a: A) -> Result<Self, Self::Error> { Ok(Self::from(a)) }
}

pub trait TryInto<A> {
    type Error;
    fn try_into(self) -> Result<A, Self::Error>;
}

impl<A: TryFrom<T>, T> TryInto<A> for T {
    type Error = A::Error;

    #[inline(always)]
    fn try_into(self) -> Result<A, Self::Error> { A::try_from(self) }
}
