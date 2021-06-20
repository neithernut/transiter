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
fn node_order_breadth_first(node: Node) -> bool {
    /// Match the ids against a sequence of (child) nodes, return the ids which
    /// may contain the next "sibling" sequences
    fn match_ids<'a>(ids: &'a [u128], reference: &[Node]) -> Option<&'a [u128]> {
        if reference.len() > 0 {
            // The generated sequence must contain a the children as a
            // contiguous sequence.
            ids.windows(reference.len())
                .position(|ids| ids.iter().zip(reference).all(|(id, node)| *id == node.id))
                .and_then(|pos| {
                    // The same must hold for grandchildren. We also reduce the
                    // search space based on the following:
                    //  * Those sequences appear only after their respecive
                    //    parents, i.e. after the "current" sequence.
                    //  * Those sequences appear in the same order as their
                    //    respective parents.
                    reference
                        .iter()
                        .try_fold(
                            ids.split_at(pos + reference.len()).1,
                            |ids, sub| match_ids(ids, sub.children.as_ref())
                        )
                        .map(|_| ids)
                })
        } else {
            Some(ids)
        }
    }

    let ids: Vec<_> = node.clone().trans_iter().breadth_first().map(|n| n.id).collect();
    match_ids(ids.as_ref(), &[node]).is_some()
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
#[derive(Clone, Debug)]
struct Node {
    id: u128,
    children: Vec<Self>,
}

impl Node {
    /// Retrieve the number of nodes
    pub fn count(&self) -> usize {
        self.children.iter().map(Self::count).sum::<usize>() + 1
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

