use js_sys::Uint8Array;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "dexie")]
extern "C" {
    type Dexie;

    type Version;

    type Collection;

    type Table;

    type WhereClause;

    type QueryValue;

    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Dexie;

    #[wasm_bindgen(method)]
    pub fn version(this: &Dexie, number: u32) -> Version;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Dexie, table_name: &str) -> Table;

    #[wasm_bindgen(method)]
    async fn add(this: &Table, data: Uint8Array, key: JsValue);

    #[wasm_bindgen(method)]
    pub fn stores(this: &Version, schema: JsValue);

    #[wasm_bindgen(method, js_name = "where")]
    pub fn where_(this: &Table, field: &str) -> WhereClause;

    #[wasm_bindgen(method, js_name = "equals")]
    pub fn equals_string(this: &WhereClause, field: &str) -> Collection;

    #[wasm_bindgen(method)]
    pub fn above(this: &WhereClause, field: u32) -> Collection;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub async fn to_array(this: &Collection) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct Schema {
    tree: String
}

#[derive(Serialize, Deserialize)]
struct Tree {
    pub id: u32,
    pub data: Vec<u8>
}

#[wasm_bindgen]
pub async fn test() {
    crate::utils::set_panic_hook();

    let dexie = Dexie::new("test".to_string());

    let schema = JsValue::from_serde(&Schema { 
        tree: "".to_string()
    }).unwrap();

    dexie.version(1).stores(schema);

    let table = dexie.get("tree");

    let test = "test".as_bytes().into();

    table.add(test, 10.into()).await;
}
