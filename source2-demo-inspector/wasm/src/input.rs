use crate::{MessageInspector, ReplayActivityIndex, ReplaySession};
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub(crate) struct ReplayBytes(Rc<Vec<u8>>);

impl From<Vec<u8>> for ReplayBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(Rc::new(bytes))
    }
}

impl AsRef<[u8]> for ReplayBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Deref for ReplayBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

#[wasm_bindgen]
pub struct ReplayInput {
    bytes: Vec<u8>,
    size: usize,
}

#[wasm_bindgen]
impl ReplayInput {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Result<Self, JsError> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|error| JsError::new(&error.to_string()))?;
        Ok(Self { bytes, size })
    }

    pub fn append(&mut self, chunk: &[u8]) -> Result<(), JsError> {
        if chunk.len() > self.size - self.bytes.len() {
            return Err(JsError::new("Replay data exceeds its expected size."));
        }
        self.bytes.extend_from_slice(chunk);
        Ok(())
    }

    #[wasm_bindgen(js_name = "intoSession")]
    pub fn into_session(self) -> Result<ReplaySession, JsError> {
        ReplaySession::from_bytes(self.finish()?)
    }

    #[wasm_bindgen(js_name = "intoMessages")]
    pub fn into_messages(self) -> Result<MessageInspector, JsError> {
        MessageInspector::from_bytes(self.finish()?)
    }

    #[wasm_bindgen(js_name = "intoActivity")]
    pub fn into_activity(self) -> Result<ReplayActivityIndex, JsError> {
        ReplayActivityIndex::new(self.finish()?)
    }
}

impl ReplayInput {
    fn finish(self) -> Result<Vec<u8>, JsError> {
        if self.bytes.len() != self.size {
            return Err(JsError::new("Replay data is incomplete."));
        }
        Ok(self.bytes)
    }
}
