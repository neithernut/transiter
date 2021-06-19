//! Transitive iterator and utilities

use std::iter::FromIterator;


/// Transitive iterator
///
/// The iterator yields all elements which are transitively reachable from an
/// initial set of items through a given recursion function, including those
/// initial items. Items discovered through a call to the recursion function
/// will only be yielded after the item passed in that call. I.e. if the
/// recursion function yields the "children" of a node, a node will only be
/// yielded after its "parent".
///
/// By default, the iterator will yield siblings, i.e. the items yielded by a
/// single call to the recursion function, grouped together. This behavior can
/// be changed by calling `depth_first` or `depth_first_unordered`.
///
/// Note that the iterator itself will not filter items which are reachable via
/// multiple paths. Generally, this iterator is not suitable for navigating
/// potentially cyclic structures on its own. For such structures, consider
/// implementing the necessary filtering in the recursion function supplied
/// during iterator creation.
///
/// # Example
///
/// ```
/// let names: Vec<_> = transiter::TransIter::new(
///     String::new(),
///     |s| { let s = s.clone(); ["a", "b", "c"].iter().map(move |c| s.clone() + c)}
/// ).take(10).collect();
/// assert_eq!(names, vec!["", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc"]);
/// ```
#[derive(Clone, Debug)]
pub struct TransIter<F: FnMut(&T) -> I, I: IntoIterator<Item = T>, T> {
    get_next: F,
    queue: std::collections::VecDeque<T>,
    mode: Mode,
}

impl<F: FnMut(&T) -> I, I: IntoIterator<Item = T>, T> TransIter<F, I, T> {
    /// Create a new transitive iterator
    ///
    /// The iterator will yield all elements which are transitively reachable
    /// from the `initial` item through the given `recursion` function,
    /// including the `initial` itself.
    pub fn new(initial: T, recursion: F) -> Self {
        Self {get_next: recursion, queue: std::iter::once(initial).collect(), mode: Mode::BreadthFirst}
    }

    /// Create a new transitive iterator with multiple initial items
    ///
    /// The iterator will yield all elements which are transitively reachable
    /// from the `initial` set of items through the given `recursion` function,
    /// including the items in the initial set.
    pub fn new_multi(initial: impl IntoIterator<Item = T>, recursion: F) -> Self {
        Self {get_next: recursion, queue: FromIterator::from_iter(initial), mode: Mode::BreadthFirst}
    }

    /// Make this iterator iterate breadth first
    ///
    /// The iterator will yield siblings grouped together, in the order they
    /// were yielded by the `Iterator` returned by the recursion function.
    ///
    /// This is the default mode.
    pub fn breadth_first(self) -> Self {
        Self {mode: Mode::BreadthFirst, ..self}
    }

    /// Make this iterator iterate depth first
    ///
    /// After yielding an item, the iterator will yield all the items reachable
    /// from that item before yielding the items next sibling.
    ///
    /// Siblings will be yielded in the order they were yielded by the
    /// `Iterator` returned by the recursion function. Note that preserving the
    /// order inhibits some additional cost. Consider using
    /// `depth_first_unordered` instead.
    pub fn depth_first(self) -> Self {
        Self {mode: Mode::DepthFirst, ..self}
    }

    /// Make this iterator iterate depth first, without preserving sibling order
    ///
    /// After yielding an item, the iterator will yield all the items reachable
    /// from that item before yielding the items next sibling.
    ///
    /// The order of the siblings is not preserved, i.e. it may differ from the
    /// order they were yielded by the `Iterator` returned by the recursion
    /// function.
    pub fn depth_first_unordered(self) -> Self {
        Self {mode: Mode::DepthFirstUnordered, ..self}
    }
}

impl<F: FnMut(&T) -> I, I: IntoIterator<Item = T>, T> Iterator for TransIter<F, I, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let res = self.queue.pop_front();
        res.as_ref().map(&mut self.get_next).map(|items| match self.mode {
            Mode::BreadthFirst          => self.queue.extend(items),
            Mode::DepthFirst            => {
                let mut items = Vec::from_iter(items);
                self.queue.reserve(items.len());
                while let Some(i) = items.pop() {
                    self.queue.push_front(i);
                }
            },
            Mode::DepthFirstUnordered   => {
                let items = items.into_iter();
                self.queue.reserve(items.size_hint().0);
                items.for_each(|i| self.queue.push_front(i))
            },
        });

        res
    }
}


#[derive(Copy, Clone, Debug)]
enum Mode {
    BreadthFirst,
    DepthFirst,
    DepthFirstUnordered,
}


/// Create a `TransIter` directly from some value
///
/// This trait defines the function `trans_iter_with` which, when called on a
/// value, returns a `TransIter` with an initial set derived from that value.
///
/// # Example
///
/// ```
/// use transiter::IntoTransIter;
///
/// let names: Vec<_> = String::new()
///     .trans_iter_with(|s| { let s = s.clone(); ["a", "b", "c"].iter().map(move |c| s.clone() + c)})
///     .take(10)
///     .collect();
/// assert_eq!(names, vec!["", "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc"]);
/// ```
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


/// Create a `TransIter` directly from some value, with type-specific recursion
///
/// This trait defines the function `trans_iter` which, when called on a value,
/// returns a `TransIter` with an initial set derived from that value.
///
/// Users may implement this trait for types with inherent and/or obvious
/// relations to other items of the same type such as recursive/tree-like
/// structures.
pub trait AutoTransIter<T>: IntoTransIter<T> + Sized {
    /// Type of the iterator returned by `recurse`
    type RecIter: IntoIterator<Item = T>;

    /// Retrieve the "next" items reachable from a given item
    fn recurse(item: &T) -> Self::RecIter;

    /// Create a `TransIter` from this value
    ///
    /// Create a `TransIter` with an initial set derived from this value and the
    /// type specific recursion function.
    fn trans_iter(self) -> TransIter<fn(&T) -> Self::RecIter, Self::RecIter, T> {
        self.trans_iter_with(Self::recurse)
    }
}

