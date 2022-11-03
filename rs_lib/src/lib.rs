use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn concat_strings(strings: Array) -> JsString {
    strings.join(&" ")
}

// Called when the wasm module is instantiated...not working
#[wasm_bindgen(start)]
pub fn init_dom() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let document = web_sys::window()
        .unwrap()
        .document()
        .expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let heading = document.create_element("h1")?;
    heading.set_text_content(Some("Hello from Rust!"));

    let para = document.create_element("p")?;
    para.set_inner_html("<strong>Bold</strong> moves");

    [heading, para].iter().for_each(|element| {
        body.append_child(&element).unwrap();
    });

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn greet(name: JsString) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(module = "/js/foo.js")]
extern "C" {
    fn js_add(a: u32, b: u32) -> u32;
}
#[wasm_bindgen]
pub fn add2(a: u32, b: u32) -> u32 {
    js_add(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }
}
