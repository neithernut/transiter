//! Shortest path: minimum length/cost
//!
//! This example demonstrates the implementation of a dijkstra-like algorithm
//! using a `TransPrioQueue`. The algorithm operates on a number of waypoints
//! under the assumption that we can hop to any waypoint as long as it's "in
//! range", i.e. the distance is lower than some threshold.

use std::fmt;


/// Waypoint
#[derive(Copy, Clone, PartialEq)]
struct Node(&'static str, i32, i32);

impl Node {
    pub fn distance(&self, other: &Self) -> u32 {
        let x = (other.1 - self.1) as f32;
        let y = (other.2 - self.2) as f32;
        (x*x + y*y).sqrt() as u32
    }
}

impl transiter::IntoTransIter<Path> for Node {
    fn trans_iter_with<F: FnMut(&Path) -> I, I: IntoIterator<Item = Path>>(
        self,
        recursion: F
    ) -> transiter::TransIter<F, I, Path> {
        Path::new(self).trans_iter_with(recursion)
    }
}


/// Path
#[derive(Clone)]
struct Path {
    data: Vec<Node>
}

impl Path {
    /// Create a new path with a starting [Node]
    pub fn new(first: Node) -> Self {
        Self {data: vec![first]}
    }

    /// Retrieve the last/current [Node]
    pub fn last(&self) -> Node {
        self.data.last().unwrap().clone()
    }

    /// Create a version of this path extended with the given [Node]
    pub fn with(&self, next: Node) -> Self {
        let mut data = self.data.clone();
        data.push(next);
        Self {data}
    }

    /// Retrieve the length of this path
    pub fn len(&self) -> u32 {
        self.data.windows(2).map(|p| p[0].distance(&p[1])).sum()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().try_for_each(|n| n.0.fmt(f))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&other.len(), &self.len())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&other.len(), &self.len())
    }
}

impl Eq for Path {}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.len(), &other.len())
    }
}


fn main() {
    use transiter::IntoTransIter;

    let mut nodes = vec![
        Node("A", 45, 59),
        Node("B", 68, 69),
        Node("C", 32, 78),
        Node("D", 15, 65),
        Node("E", 45, 12),
        Node("F", 98, 80),
    ];

    let range = 50;

    // We are looking for the path from 'S' to 'G' with the minimum path length.
    // We do so by using a `TransIter` over `Path`s with a recursion function
    // which extends the given path with a node which is in range of the last
    // node. If the destination is reachable, the iterator will eventually yield
    // a path with the destination as its last node.
    let path = Node("S", 0, 0)
        .trans_prio_queue_with(move |path: &Path| {
            let current = path.last();
            let in_range = |next: &Node| current.distance(next) < range;
            let res: Vec<_> = nodes.iter().filter(|n| in_range(n)).map(|n| path.with(*n)).collect();
            nodes.retain(|n| !in_range(n));
            res
        })
        .inspect(|path| eprintln!("{} {}", path, path.len()))
        .find(|path| path.last().0 == "F")
        .expect("Could not find path");

    println!("S->F: {}, length: {}", path, path.len());
}

