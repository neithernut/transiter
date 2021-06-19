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

impl AutoTransIter<Node> for Node {
    type RecIter = Vec<Self>;

    fn recurse(item: &Self) -> Self::RecIter {
        item.children.clone()
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

