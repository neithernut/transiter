//! Transitive iterator and utilities


/// Transitive iterator
///
/// The iterator yields all elements which are transitively reachable from an
/// initial set of items through a given recursion function, including those
/// initial items. Items discovered through a call to the recursion function
/// will only be yielded after the item passed in that call. I.e. if the
/// recursion function yields the "children" of a node, a node will only be
/// yielded after its "parent".
#[derive(Clone, Debug)]
pub struct TransIter<F: FnMut(&T) -> I, I: IntoIterator<Item = T>, T> {
    get_next: F,
    queue: std::collections::VecDeque<T>,
}

impl<F: FnMut(&T) -> I, I: IntoIterator<Item = T>, T> TransIter<F, I, T> {
    /// Create a new transitive iterator
    ///
    /// The iterator will yield all elements which are transitively reachable
    /// from the `initial` item through the given `recursion` function,
    /// including the `initial` itself.
    pub fn new(initial: T, recursion: F) -> Self {
        Self {get_next: recursion, queue: std::iter::once(initial).collect()}
    }

    /// Create a new transitive iterator with multiple initial items
    ///
    /// The iterator will yield all elements which are transitively reachable
    /// from the `initial` set of items through the given `recursion` function,
    /// including the items in the initial set.
    pub fn new_multi(initial: impl IntoIterator<Item = T>, recursion: F) -> Self {
        Self {get_next: recursion, queue: std::iter::FromIterator::from_iter(initial)}
    }
}

impl<F: FnMut(&T) -> I, I: IntoIterator<Item = T>, T> Iterator for TransIter<F, I, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let res = self.queue.pop_front();
        res.as_ref().map(&mut self.get_next).map(|items| self.queue.extend(items));

        res
    }
}


/// Create a `TransIter` directly from some value
///
/// This trait defines the function `trans_iter_with` which, when called on a
/// value, returns a `TransIter` with an initial set derived from that value.
pub trait IntoTransIter<T> {
    /// Create a `TransIter` from this value
    ///
    /// Create a `TransIter` with an initial set derived from this value and
    /// the given recursion function.
    fn trans_iter_with<F: FnMut(&T) -> I, I: IntoIterator<Item = T>>(
        self,
        recursion: F
    ) -> TransIter<F, I, T>;
}

impl<T> IntoTransIter<T> for T {
    fn trans_iter_with<F: FnMut(&T) -> I, I: IntoIterator<Item = T>>(
        self,
        recursion: F
    ) -> TransIter<F, I, T> {
        TransIter::new(self, recursion)
    }
}

