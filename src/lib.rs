use wasm_bindgen::prelude::*;
use web_sys::console;

mod cm_rhai_mode;
mod codemirror;
mod scripting;

#[wasm_bindgen]
pub fn run_script(
    script: String,
    print_callback: js_sys::Function,
    debug_callback: js_sys::Function,
) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();

    Ok(scripting::run_script(
        &script,
        move |s| {
            let _ = print_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
        move |s| {
            let _ = debug_callback.call1(&JsValue::null(), &JsValue::from_str(s));
        },
    )?)
}

#[wasm_bindgen]
pub fn compile_script(script: String) -> Result<String, JsValue> {
    Ok(scripting::compile_ast(&script)?)
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
