A draining `Iterator` which stops when a predicate becomes false.

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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
