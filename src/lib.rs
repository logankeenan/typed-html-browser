// src/lib.rs

extern crate wasm_bindgen;
extern crate typed_html;

use wasm_bindgen::prelude::*;
use typed_html::dom::DOMTree;
use typed_html::html;

#[wasm_bindgen]
pub fn create_body() -> String {
    let doc: DOMTree<String> = html!(
    <section>
        <h1>
            "typed-html browser"
        </h1>
        <p>"This was rendered using typed-html in the browser with Web Assembly!"</p>
    </section>
);
    return doc.to_string();
}