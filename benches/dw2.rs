use easybench::*;
use std::iter::Peekable;
use std::vec::Drain;

pub struct DrainWhile<'a, T: 'a, P> {
    inner: Peekable<Drain<'a,T>>,
    pred: P,
}

impl<'a,T,P> Iterator for DrainWhile<'a,T,P> where P: Fn(&T) -> bool {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let ok = if let Some(x) = self.inner.peek() {
            (self.pred)(x)
        } else { return None; };
        if ok { self.inner.next() } else { None }
    }
}

impl<'a,T,P> DrainWhile<'a,T,P> {
    pub fn from_vec(x: &'a mut Vec<T>, pred: P) -> DrainWhile<T,P> where P: Fn(&T) -> bool {
        // This is purely a performance optimisation for the 0-matching case.
        let some_match = match x.first() {
            Some(x) => pred(x),
            _ => false
        };
        if some_match {
            DrainWhile { inner: x.drain(..).peekable(), pred: pred }
        } else {
            /* none of them matched */
            DrainWhile{ inner: x.drain(0..0).peekable(), pred: pred }
        }
    }
}

pub fn bench() {
    let vec = (0..100).into_iter().zip((0..100).into_iter()).collect::<Vec<(usize,usize)>>();
    fn bench_fn<F>(n: usize, f: F, xs: &mut Vec<(usize,usize)>)
            where F: Fn(&(usize,usize)) -> bool {
        let mut t = 0;
        for (_,x) in DrainWhile::from_vec(xs, f) { t += x; }
        assert_eq!(t, n);
    }

    println!("none     {}", bench_env(vec.clone(), |xs| bench_fn(0,    |&(_,_)| false , xs)));
    println!("25 fired {}", bench_env(vec.clone(), |xs| bench_fn(300,  |&(x,_)| x < 25, xs)));
    println!("50 fired {}", bench_env(vec.clone(), |xs| bench_fn(1225, |&(x,_)| x < 50, xs)));
    println!("75 fired {}", bench_env(vec.clone(), |xs| bench_fn(2775, |&(x,_)| x < 75, xs)));
    println!("all      {}", bench_env(vec.clone(), |xs| bench_fn(4950, |&(_,_)| true  , xs)));
}
