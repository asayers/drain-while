use drain_while::*;
use easybench::*;

pub fn bench() {
    let vec = (0..100).into_iter().zip((0..100).into_iter()).collect::<Vec<(usize,usize)>>();
    fn bench_fn<F>(n: usize, f: F, xs: &mut Vec<(usize,usize)>)
            where F: FnMut(&(usize,usize)) -> bool {
        let mut t = 0;
        for (_,x) in xs.drain_while(f) { t += x; }
        assert_eq!(t, n);
    }

    println!("none     {}", bench_env(vec.clone(), |xs| bench_fn(0,    |&(_,_)| false , xs)));
    println!("25 fired {}", bench_env(vec.clone(), |xs| bench_fn(300,  |&(x,_)| x < 25, xs)));
    println!("50 fired {}", bench_env(vec.clone(), |xs| bench_fn(1225, |&(x,_)| x < 50, xs)));
    println!("75 fired {}", bench_env(vec.clone(), |xs| bench_fn(2775, |&(x,_)| x < 75, xs)));
    println!("all      {}", bench_env(vec.clone(), |xs| bench_fn(4950, |&(_,_)| true  , xs)));
}
