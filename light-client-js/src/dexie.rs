use std::marker::PhantomData;

use futures::executor::*;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tendermint::block::Height;
use tendermint_light_client::{errors::{Error, ErrorKind}, types::LightBlock};
use tendermint_testgen::{
    light_block::{LightBlock as TestgenLightBlock, TmLightBlock},
    Generator
};
use wasm_bindgen::prelude::*;

pub mod dexie {
    use js_sys::Uint8Array;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "dexie")]
    extern "C" {
        pub type Dexie;

        pub type Version;

        pub type Collection;

        #[derive(Clone, Debug)]
        pub type Table;

        pub type WhereClause;

        pub type QueryValue;

        #[wasm_bindgen(constructor)]
        pub fn new(name: &str) -> Dexie;

        #[wasm_bindgen(method)]
        pub fn version(this: &Dexie, number: u32) -> Version;

        #[wasm_bindgen(method)]
        pub fn stores(this: &Version, schema: JsValue);

        #[wasm_bindgen(method, structural, indexing_getter)]
        pub fn get(this: &Dexie, table_name: &str) -> Table;

        #[wasm_bindgen(method, js_name = "toCollection")]
        pub fn to_collection(this: &Table) -> Collection;

        #[wasm_bindgen(method)]
        pub async fn add(this: &Table, data: Uint8Array, key: Uint8Array);

        #[wasm_bindgen(method)]
        pub async fn delete(this: &Table, key: Uint8Array);

        #[wasm_bindgen(method, js_name = "where")]
        pub fn where_(this: &Table, field: &str) -> WhereClause;

        #[wasm_bindgen(method, js_name = "equals")]
        pub fn equals_array(this: &WhereClause, field: Uint8Array) -> Collection;

        #[wasm_bindgen(method, js_name = "equals")]
        pub fn equals_string(this: &WhereClause, field: String) -> Collection;

        #[wasm_bindgen(method, js_name = "equals")]
        pub fn equals_int(this: &WhereClause, field: u64) -> Collection;

        #[wasm_bindgen(method)]
        pub fn above(this: &WhereClause, field: u32) -> Collection;

        #[wasm_bindgen(method, js_name = "toArray")]
        pub async fn to_array(this: &Collection) -> JsValue;

        #[wasm_bindgen(method)]
        pub async fn first(this: &Collection) -> JsValue;
    }
}


#[derive(Serialize, Deserialize)]
struct Schema<'a> {
    tree: &'a str
}

#[derive(Clone, Debug)]
pub struct HeightIndexedWebDb<V> {
    table: dexie::Table,
    marker: PhantomData<V>
}

fn key_bytes(height: Height) -> Uint8Array {
    let slice = &height.value().to_be_bytes()[..];
    slice.into()
}

impl<V> HeightIndexedWebDb<V>
    where
    V: Serialize + DeserializeOwned,
{
    pub fn new(name: &str) -> Self {
        let dexie = dexie::Dexie::new("test");

        let schema = JsValue::from_serde(&Schema {
            tree: ""
        }).unwrap();

        dexie.version(1).stores(schema);

        Self {
            table: dexie.get("tree"),
            marker: PhantomData
        }
    }

    pub fn get(&self, height: Height) -> Result<Option<V>, Error>{
        let key = key_bytes(height);

        let js_value: JsValue = block_on(self.table.where_(":id").equals_array(key).first());

        let value = if js_value.is_undefined() {
            None
        } else {
            Some(Uint8Array::from(js_value))
        };

        match value {
            Some(js_bytes) => {
                let value =
                    serde_cbor::from_slice(&js_bytes.to_vec()).map_err(|e| ErrorKind::Store.context(e))?;
                Ok(value)
            }
            None => Ok(None),
        }
    }

    pub fn contains_key(&self, height: Height) -> bool {
        let key = key_bytes(height);

        let value: JsValue = block_on(self.table.where_(":id").equals_array(key).first());

        !value.is_undefined()
    }

    pub async fn insert(&self, height: Height, value: &V) -> Result<(), Error> {
        let key = key_bytes(height);

        let data: &[u8] = &serde_cbor::to_vec(&value).map_err(|e| ErrorKind::Store.context(e))?;

        self.table.add(data.into(), key).await;

        Ok(())
    }

    pub fn remove(&self, height: Height) {
        let key = key_bytes(height);

        block_on(self.table.delete(key));
    }

    // pub fn iter(&self) -> impl DoubleEndedIterator<Item = V> {
    //     let js_values: JsValue = block_on(self.table.to_collection().to_array());

    //     let values: Vec<JsValue> = Array::from(&js_values).to_vec();

    //     let result = values.into_iter().map(|value: JsValue| {
    //         let vec = Uint8Array::from(value).to_vec();
    //         let slice: &[u8] = &vec[..];
    //         serde_cbor::from_slice(slice)
    //     }).collect();

    //     result
    // }
}


#[wasm_bindgen]
pub async fn test() -> JsValue {
    crate::utils::set_panic_hook();

    let dexie = dexie::Dexie::new("test");

    let schema = JsValue::from_serde(&Schema { 
        tree: ""
    }).unwrap();

    dexie.version(1).stores(schema);

    let table = dexie.get("tree");

    let test = "memes irados xd".as_bytes().into();

    let key = &(50 as u64).to_be_bytes()[..];

    table.add(test, key.into()).await;

    table.to_collection().to_array().await
}

pub struct LB(LightBlock);

impl From<TmLightBlock> for LB {
    fn from(lb: TmLightBlock) -> Self {
        LB(LightBlock {
            signed_header: lb.signed_header,
            validators: lb.validators,
            next_validators: lb.next_validators,
            provider: lb.provider,
        })
    }
}

#[wasm_bindgen]
pub async fn db_test() {
    let db: HeightIndexedWebDb<LightBlock> = HeightIndexedWebDb::new("testing");

    let height = Height::default();

    let LB(light_block) =
        TestgenLightBlock::new_default(1).generate().unwrap().into();

    db.insert(height, &light_block).await;
}
