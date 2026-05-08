use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[wasm_bindgen]
pub fn count_sentences(text: &str) -> usize {
    if text.trim().is_empty() {
        return 0;
    }
    text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count()
}

#[wasm_bindgen]
pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

#[wasm_bindgen]
pub fn byte_len(text: &str) -> usize {
    text.len()
}
