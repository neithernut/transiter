//! Tests

use quickcheck::{Arbitrary, Gen};

use super::*;


#[quickcheck]
fn smoke(node: Node) {
    node.trans_iter().for_each(|_| ())
}

#[quickcheck]
fn node_count_breadth_first(node: Node) -> bool {
    let count = node.count();
    node.trans_iter().breadth_first().count() == count
}

#[quickcheck]
fn node_count_depth_first(node: Node) -> bool {
    let count = node.count();
    node.trans_iter().depth_first().count() == count
}

#[quickcheck]
fn node_count_depth_first_unordered(node: Node) -> bool {
    let count = node.count();
    node.trans_iter().depth_first_unordered().count() == count
}

#[quickcheck]
fn node_count_prio_queue(node: Node) -> bool {
    let count = node.count();
    node.trans_prio_queue().count() == count
}

#[quickcheck]
fn node_order_breadth_first(node: Node) -> bool {
    /// Match the ids against a sequence of (child) nodes. The nodes are
    /// expected within the given `counts[0]` after the given offset. `counts`
    /// is expected to hold the number of nodes at a given depth.
    ///
    /// The function returns the offset at which to look for grand-siblings of
    /// the given `reference`s.
    fn match_ids(ids: &[u128], offset: usize, reference: &[Node], counts: &[usize]) -> Option<usize> {
        if reference.is_empty() {
            Some(offset)
        } else if let Some((depth_count, counts)) = counts.split_first() {
            let (ids, next) = ids.split_at(*depth_count);
            let ids = ids.split_at(offset).1;

            // The generated sequence must contain a the children as a
            // contiguous sequence.
            ids.windows(reference.len())
                .position(|ids| ids.iter().eq(reference.iter().map(|n| &n.id)))
                .and_then(|pos| {
                    // The same must hold for grandchildren. Those sequences
                    // appear in the same order as their respective parents.
                    reference
                        .iter()
                        .try_fold(0, |off, sub| match_ids(next, off, sub.children.as_ref(), counts))
                        .map(|_| offset + pos + reference.len())
                })
        } else {
            Some(offset)
        }
    }

    let ids: Vec<_> = node.clone().trans_iter().breadth_first().map(|n| n.id).collect();
    let counts: Vec<_> = (0..).map(|d| node.count_at_depth(d)).take_while(|c| *c > 0).collect();
    match_ids(ids.as_ref(), 0, &[node], counts.as_ref()).is_some()
}

#[quickcheck]
fn node_order_depth_first(node: Node) -> bool {
    /// Match the subtree with the given root node, return the remaining ids
    fn match_ids<'a>(ids: &'a [u128], root: &Node) -> Option<&'a [u128]> {
        ids.split_first()
            .and_then(|(first, ids)| if *first == root.id { Some(ids) } else { None })
            .and_then(|ids| root.children.iter().try_fold(ids, |ids, sub| match_ids(ids, sub)))
    }

    let ids: Vec<_> = node.clone().trans_iter().depth_first().map(|n| n.id).collect();
    match_ids(ids.as_ref(), &node) == Some(&[])
}

#[quickcheck]
fn node_order_depth_first_unordered(node: Node) -> bool {
    /// Match the subtree with the given root node, return the remaining ids
    fn match_ids<'a>(ids: &'a [u128], root: &Node) -> Option<&'a [u128]> {
        // While we don't advertise any order in which siblings may appear, we
        // know that with our implementation, they appear in reverse order.
        ids.split_first()
            .and_then(|(first, ids)| if *first == root.id { Some(ids) } else { None })
            .and_then(|ids| root.children.iter().try_rfold(ids, |ids, sub| match_ids(ids, sub)))
    }

    let ids: Vec<_> = node.clone().trans_iter().depth_first_unordered().map(|n| n.id).collect();
    match_ids(ids.as_ref(), &node) == Some(&[])
}


/// Dumb recursive structure for testing
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    id: u128,
    children: Vec<Self>,
}

impl Node {
    /// Retrieve the number of nodes
    pub fn count(&self) -> usize {
        self.children.iter().map(Self::count).sum::<usize>() + 1
    }

    /// Retrieve the number of nodes with a given depth
    ///
    /// The depth is `0`-based. When called with a `depth` of `0`, this function
    /// will therefore always yield `1` for any given `Node`.
    pub fn count_at_depth(&self, depth: usize) -> usize {
        if let Some(depth) = depth.checked_sub(1) {
            self.children.iter().map(|n| n.count_at_depth(depth)).sum::<usize>()
        } else {
            1
        }
    }
}

impl<'a> AutoTransIter<&'a Node> for &'a Node {
    type RecIter = std::slice::Iter<'a, Node>;

    fn recurse(item: &&'a Node) -> Self::RecIter {
        item.children.iter()
    }
}

impl Arbitrary for Node {
    fn arbitrary(g: &mut Gen) -> Self {
        let children = if g.size() > 0 {
            Arbitrary::arbitrary(&mut Gen::new(g.size() / 2))
        } else {
            Default::default()
        };
        Self {id: Arbitrary::arbitrary(g), children}
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.children.clone().into_iter())
    }
}

