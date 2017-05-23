use std::vec::Drain;

pub trait VecExt<T> {
    fn drain_while<P>(&mut self, pred: P) -> DrainWhile<T> where P: FnMut(&T) -> bool;
}

// Just a newtype to allow changing the implementation later.
pub struct DrainWhile<'a, T: 'a>(Drain<'a, T>);
impl<'a, T> Iterator for DrainWhile<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.0.next() }
}

impl<T> VecExt<T> for Vec<T> {
    // TODO: Surely this can be implemented more efficiently, but it may not be worth the effort...
    fn drain_while<P>(&mut self, mut pred: P) -> DrainWhile<T> where P: FnMut(&T) -> bool {
        // This is purely a performance optimisation for the 0-matching case.
        let some_match = match self.first() {
            Some(x) => pred(x),
            _ => false
        };
        if some_match {
            match self.iter().position(|x| !pred(x)) {
                None => /* they all matched pred */ DrainWhile(self.drain(..)),
                Some(i) => /* they matched until i */ DrainWhile(self.drain(..i)),
            }
        } else {
            /* none of them matched */ DrainWhile(self.drain(0..0))
        }
    }
}
