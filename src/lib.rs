#![no_std]

extern crate void;

use void::Void;

pub trait TryClone: Sized {
    type Error;
    fn try_clone(&self) -> Result<Self, Self::Error>;
}

impl<T: Clone> TryClone for T {
    type Error = Void;
    fn try_clone(&self) -> Result<Self, Void> { Ok(self.clone()) }
}
