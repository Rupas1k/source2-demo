use std::rc::Rc;

#[derive(Clone, Default)]
pub struct StringTableRow {
    pub(crate) index: i32,
    pub(crate) key: String,
    pub(crate) value: Option<Rc<Vec<u8>>>,
}

impl StringTableRow {
    pub(crate) fn new(index: i32, key: String, value: Option<Rc<Vec<u8>>>) -> Self {
        StringTableRow { index, key, value }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn value(&self) -> Option<&[u8]> {
        self.value.as_ref().map(|x| x.as_slice())
    }
}
