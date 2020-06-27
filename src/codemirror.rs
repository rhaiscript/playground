use js_sys::{Array, RegExp};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "codemirror")]
extern "C" {
    pub type StringStream;

    #[wasm_bindgen(method)]
    #[must_use]
    pub fn eol(this: &StringStream) -> bool;

    #[wasm_bindgen(method)]
    #[must_use]
    pub fn sol(this: &StringStream) -> bool;

    #[wasm_bindgen(method)]
    #[must_use]
    pub fn peek(this: &StringStream) -> Option<char>;

    #[wasm_bindgen(method)]
    pub fn next(this: &StringStream) -> Option<char>;

    #[wasm_bindgen(method, js_name = eat)]
    pub fn eat_char(this: &StringStream, m: char) -> bool;

    #[wasm_bindgen(method, js_name = eat)]
    pub fn eat_regexp(this: &StringStream, m: &RegExp) -> bool;

    #[wasm_bindgen(method, js_name = eat)]
    pub fn eat(this: &StringStream, m: &dyn Fn(char) -> bool) -> bool;

    #[wasm_bindgen(method, js_name = eatWhile)]
    pub fn eat_while_char(this: &StringStream, m: char) -> bool;

    #[wasm_bindgen(method, js_name = eatWhile)]
    pub fn eat_while_regexp(this: &StringStream, m: &RegExp) -> bool;

    #[wasm_bindgen(method, js_name = eatWhile)]
    pub fn eat_while(this: &StringStream, m: &dyn Fn(char) -> bool) -> bool;

    #[wasm_bindgen(method, js_name = eatSpace)]
    pub fn eat_space(this: &StringStream) -> bool;

    #[wasm_bindgen(method, js_name = skipToEnd)]
    pub fn skip_to_end(this: &StringStream);

    #[wasm_bindgen(method, js_name = skipTo)]
    pub fn skip_to(this: &StringStream, str: &str) -> bool;

    #[wasm_bindgen(method, js_name = match)]
    #[must_use]
    pub fn match_str(this: &StringStream, pattern: &str, consume: bool, case_fold: bool) -> bool;

    #[wasm_bindgen(method, js_name = match)]
    #[must_use]
    pub fn match_regexp(this: &StringStream, pattern: &RegExp, consume: bool) -> Array;

    #[wasm_bindgen(method, js_name = backUp)]
    pub fn back_up(this: &StringStream, n: u32);

    #[wasm_bindgen(method)]
    #[must_use]
    pub fn column(this: &StringStream) -> u32;

    #[wasm_bindgen(method)]
    #[must_use]
    pub fn indentation(this: &StringStream) -> u32;

    #[wasm_bindgen(method)]
    #[must_use]
    pub fn current(this: &StringStream) -> String;

    #[wasm_bindgen(method, js_name = lookAhead)]
    #[must_use]
    pub fn look_ahead(this: &StringStream, n: u32) -> Option<String>;

    // #[wasm_bindgen(method, js_name = baseToken)]
    // #[must_use]
    // pub fn base_token(this: &StringStream) -> JsValue;
}

// Note: JavaScript fields must not be imported using this method when this
//       WASM module is also to be used in Web Workers. This method will cause
//       wasm-bindgen to pull in the referenced module even when in a Web
//       Worker, which will cause serious code bloat even if it doesn't error
//       out. The alternative is to provide an initializer function that is
//       called from JavaScript when used in the page context. (In this case
//       the function is `cm_rhai_mode::init_codemirror_pass`.)
// #[wasm_bindgen(module = "codemirror")]
// extern "C" {
//     #[wasm_bindgen(getter, js_name = Pass)]
//     pub static CODEMIRROR_PASS: JsValue;
// }
