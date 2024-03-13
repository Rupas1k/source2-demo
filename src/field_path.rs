#[derive(Clone, Debug)]
pub struct FieldPath {
    pub(crate) path: [i32; 7],
    pub(crate) last: usize,
}

impl FieldPath {
    pub(crate) fn new() -> Self {
        FieldPath {
            path: [-1, 0, 0, 0, 0, 0, 0],
            last: 0,
        }
    }
    pub fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.path[self.last] = 0;
            self.last -= 1;
        }
    }
}
