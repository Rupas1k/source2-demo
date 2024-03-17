use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EHTree {
    Leaf {
        weight: i32,
        value: i32,
    },
    Node {
        weight: i32,
        value: i32,
        left: Box<EHTree>,
        right: Box<EHTree>,
    },
}

impl Ord for EHTree {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight().cmp(&other.weight()) {
            Ordering::Equal => self.value().cmp(&other.value()),
            ord => ord.reverse(),
        }
    }
}

impl PartialOrd for EHTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl EHTree {
    pub fn weight(&self) -> i32 {
        match self {
            EHTree::Leaf { weight, .. } | EHTree::Node { weight, .. } => *weight,
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            EHTree::Leaf { value, .. } | EHTree::Node { value, .. } => *value,
        }
    }

    pub fn left(&self) -> &EHTree {
        match self {
            EHTree::Node { left, .. } => left,
            EHTree::Leaf { .. } => panic!(),
        }
    }

    pub fn right(&self) -> &EHTree {
        match self {
            EHTree::Node { right, .. } => right,
            EHTree::Leaf { .. } => panic!(),
        }
    }
}

pub fn build_huffman_tree(freqs: Vec<i32>) -> Option<EHTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut trees = freqs
        .iter()
        .enumerate()
        .map(|(v, w)| EHTree::Leaf {
            value: v as i32,
            weight: if *w == 0 { 1 } else { *w },
        })
        .collect::<BinaryHeap<EHTree>>();

    let mut n = 40;

    while trees.len() > 1 {
        let a = trees.pop().unwrap();
        let b = trees.pop().unwrap();

        trees.push(EHTree::Node {
            weight: a.weight() + b.weight(),
            value: n,
            left: Box::new(a),
            right: Box::new(b),
        });

        n += 1;
    }

    Some(trees.pop().unwrap())
}
