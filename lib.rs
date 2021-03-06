/*!
This library provides a draining `Iterator` which stops when a predicate becomes false.

```
use drain_while::*;

let mut original = vec![1,2,3,4,5];
let mut matching = vec![];

for x in original.drain_while(|x| *x < 3) {
    matching.push(x);
}

assert_eq!(matching, vec![1,2]);
assert_eq!(original, vec![3,4,5]);
```

See the documentation for [`drain_while()`](trait.DrainWhileable.html#tymethod.drain_while) for
more.
*/

use std::vec::Drain;

pub trait DrainWhileable<T> {
    /// Take the elements of a vector, left-to-right, stopping at the first non-matching element.
    ///
    /// The returned `DrainWhile` iterates over the longest prefex in which all elements satisfy
    /// `pred`; when the iterator is dropped, the prefix is removed from `vec`.
    ///
    /// ```
    /// # use drain_while::*;
    /// let mut orig: Vec<usize> = vec![0,1,2,3,4,5];
    /// let none: Vec<usize> = orig.drain_while(|_| false).collect();
    /// let some: Vec<usize> = orig.drain_while(|x| *x < 3).collect();
    /// let rest: Vec<usize> = orig.drain_while(|_| true).collect();
    ///
    /// assert_eq!(none, vec![]);
    /// assert_eq!(some, vec![0,1,2]);
    /// assert_eq!(rest, vec![3,4,5]);
    /// assert_eq!(orig, vec![]);
    /// ```
    ///
    /// The behaviour of `drain_while()` differs from `drain().take_while()` in the final state of
    /// the original vector, as illustrated here:
    ///
    /// ```
    /// # use drain_while::*;
    /// let mut v1 = vec![1,2,3,4,5];
    /// let mut v2 = vec![1,2,3,4,5];
    /// v1.drain(..).take_while(|x| *x < 3);
    /// v2.drain_while(|x| *x < 3);
    /// assert_eq!(v1, vec![]);
    /// assert_eq!(v2, vec![3,4,5]);
    /// ```
    ///
    /// The same caveats which apply to `drain` also apply to `drain_while`:
    ///
    /// 1. The element range is removed even if the iterator is only partially consumed or not
    ///    consumed at all.
    /// 2. It is unspecified how many elements are removed from the vector, if the `DrainWhile`
    ///    value is leaked.
    ///
    /// The current implementation is fairly naive, but I believe there's scope for speeding it up
    /// substantially.
    fn drain_while<P>(&mut self, pred: P) -> DrainWhile<T> where P: Fn(&T) -> bool;
}

/// A draining iterator for `Vec<T>`.
///
/// See [`Vec::drain_while`](trait.DrainWhileable.html#tymethod.drain_while) for more.
//
// Just a newtype to allow changing the implementation later.
pub struct DrainWhile<'a, T: 'a>(Drain<'a, T>);
impl<'a, T> Iterator for DrainWhile<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.0.next() }
}

impl<T> DrainWhileable<T> for Vec<T> {
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
