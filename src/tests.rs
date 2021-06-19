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

