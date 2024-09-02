use crate::reader::field::OPERATIONS;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum HTree {
    Leaf {
        weight: i32,
        value: i32,
    },
    Node {
        weight: i32,
        value: i32,
        left: Box<HTree>,
        right: Box<HTree>,
    },
}

impl Default for HTree {
    fn default() -> Self {
        let mut trees = OPERATIONS
            .map(|(_, weight)| weight)
            .iter()
            .enumerate()
            .map(|(v, w)| HTree::Leaf {
                value: v as i32,
                weight: if *w == 0 { 1 } else { *w },
            })
            .collect::<BinaryHeap<HTree>>();

        let mut n = 40;

        while trees.len() > 1 {
            let a = trees.pop().unwrap();
            let b = trees.pop().unwrap();

            trees.push(HTree::Node {
                weight: a.weight() + b.weight(),
                value: n,
                left: Box::new(a),
                right: Box::new(b),
            });

            n += 1;
        }

        trees.pop().unwrap()
    }
}

impl Ord for HTree {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight().cmp(&other.weight()) {
            Ordering::Equal => self.value().cmp(&other.value()),
            ord => ord.reverse(),
        }
    }
}

impl PartialOrd for HTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HTree {
    #[inline]
    pub fn weight(&self) -> i32 {
        match self {
            HTree::Leaf { weight, .. } | HTree::Node { weight, .. } => *weight,
        }
    }

    #[inline]
    pub fn value(&self) -> i32 {
        match self {
            HTree::Leaf { value, .. } | HTree::Node { value, .. } => *value,
        }
    }

    #[inline]
    pub fn left(&self) -> &HTree {
        match self {
            HTree::Node { left, .. } => left,
            HTree::Leaf { .. } => unreachable!(),
        }
    }

    #[inline]
    pub fn right(&self) -> &HTree {
        match self {
            HTree::Node { right, .. } => right,
            HTree::Leaf { .. } => unreachable!(),
        }
    }
}
