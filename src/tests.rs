//! Tests

use quickcheck::{Arbitrary, Gen};


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

