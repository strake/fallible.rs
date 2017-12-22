use void::Void;

pub trait TryIterator {
    type Item;
    type Error;
    fn try_next(&mut self) -> Result<Option<Self::Item>, Self::Error>;
}

impl<I: Iterator> TryIterator for I {
    type Item = <I as Iterator>::Item;
    type Error = Void;
    #[inline]
    fn try_next(&mut self) -> Result<Option<Self::Item>, Void> { Ok(self.next()) }
}
