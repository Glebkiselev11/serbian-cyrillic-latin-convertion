mod translate;

pub use translate::Convertion;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Transliterator(Convertion);

#[wasm_bindgen]
impl Transliterator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Convertion {})
    }

    #[wasm_bindgen]
    pub fn to_cyrillic(&self, text: &str) -> String {
        Convertion::from_latin(text)
    }

    #[wasm_bindgen]
    pub fn to_latin(&self, text: &str) -> String {
        Convertion::from_cyrillic(text)
    }
}
