# TransIter -- transitive iterator and utilities

This small rust crate provides `TransIter`, an iterator suitable for navigating
recursive structures and DAGs. The iterator allows for multiple modes of
iteration. For structures in which the nodes implement `Ord`, this crate also
provides a `TransPrioQueue`.

In addition to the iterators themselves, this crate provides some convenience
traits for creating instances of those iterators.

## Example

```rust
use transiter::IntoTransIter;

let names: Vec<_> = String::new()
    .trans_iter_with(|s| { let s = s.clone(); ["a", "b", "c"].iter().map(move |c| s.clone() + c)})
    .take(10)
    .collect();
assert_eq!(names, vec!["", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc"]);
```

## Similar crates

The following crates serve a similar purpose:

 * [reciter](https://crates.io/crates/reciter) provides a macro for creating an
   iterator from a recursive function.

## License

This work is provided under the MIT license. See `LICENSE` for more details.

