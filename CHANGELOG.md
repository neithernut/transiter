# 0.2.0 -- 2021-10-31

## Added
- A hint in the documentation about implementing `AutoTransIter` for references.
- `TransPrioQueue`, a transitive priority queue, and functions for creating such
  a queue from a `TransIter`, including documentation and a test.
- Examples illustrating the use of the library's iterators for shortest-path
  searches.

## Changed
- Items referred to in documentation text are now actual references.
- A confusing piece of documentation regarding a blanket implementation was
  rewritten.
- The test `tests::node_order_breadth_first` was rewritten for better
  performance (but still takes significantly more time than any other test).


# 0.1.0 -- 2021-06-20

## Added
- `TransIter`, `IntoTransIter` and `AutoTransiter`, including documentation and
  tests.
