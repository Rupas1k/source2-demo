use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct EntityRecord {
    pub index: i32,
    pub name: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct EntityFieldRecord {
    pub path: Vec<u16>,
    #[wasm_bindgen(js_name = "namedPath")]
    pub named_path: Vec<String>,
    pub value: String,
    #[wasm_bindgen(js_name = "encodedAs")]
    pub encoded_as: String,
    #[wasm_bindgen(js_name = "decodedAs")]
    pub decoded_as: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct TableRecord {
    pub index: i32,
    pub name: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct TableEntryRecord {
    pub index: i32,
    pub string: Option<Vec<u8>>,
    #[wasm_bindgen(js_name = "userData")]
    pub user_data: Option<Vec<u8>>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct EventRecord {
    pub id: i32,
    pub name: String,
}

#[wasm_bindgen(getter_with_clone)]
pub struct EventKeyRecord {
    pub index: i32,
    pub name: String,
    #[wasm_bindgen(js_name = "typeId")]
    pub type_id: i32,
    #[wasm_bindgen(js_name = "typeName")]
    pub type_name: String,
}
