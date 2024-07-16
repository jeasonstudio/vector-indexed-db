use hnsw::{Hnsw, Searcher};
use space::{Metric, Neighbor};
use wasm_bindgen::prelude::*;

#[derive(Default, Clone)]
#[wasm_bindgen()]
pub struct VectorIndexedDB {
    pub foo123: bool,
}

#[wasm_bindgen]
pub fn vector_indexed_db_new() -> VectorIndexedDB {
    VectorIndexedDB::default()
}
