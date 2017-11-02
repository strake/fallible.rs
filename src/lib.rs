#![no_std]

extern crate void;

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
