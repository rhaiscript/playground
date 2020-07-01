use rhai::{
    packages::{EvalPackage, Package},
    ParseError,
};
use wasm_bindgen::JsValue;
use web_sys::console;

pub fn run_script(
    script: &str,
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
    progress_callback: impl Fn(u64) + 'static,
) -> Result<String, String> {
    let engine = {
        let mut engine = rhai::Engine::new();
        engine.on_progress(move |&ops| {
            // TODO: Automatically adjust interval according to speed
            let interval = if cfg!(debug_assertions) {
                100_000
            } else {
                1_000_000
            };
            if ops % interval == 0 {
                progress_callback(ops);
            }
            true
        });
        engine.load_package(EvalPackage::new().get());
        engine.on_print(move |s| print_callback(s));
        engine.on_debug(move |s| debug_callback(s));
        engine
    };
    let script_ast = engine.compile(&script).map_err(|e| e.to_string())?;

    let result: rhai::Dynamic = engine.eval_ast(&script_ast).map_err(|e| e.to_string())?;
    Ok(result.to_string())
}

thread_local! {
    static ENGINE_FOR_AST_ONLY: rhai::Engine = rhai::Engine::new();
}

pub fn compile_ast(script: &str) -> Result<String, JsValue> {
    ENGINE_FOR_AST_ONLY.with(|engine| {
        let script_ast = engine.compile(&script).map_err(parse_error_to_js)?;
        console::log_1(&JsValue::from_str("Script compiled to AST!"));
        Ok("".into())
    })
}

#[derive(serde::Serialize)]
struct OutParseError {
    message: String,
    line: Option<u32>,
    column: Option<u32>,
}

fn parse_error_to_js(e: ParseError) -> JsValue {
    let ParseError(err, pos) = e;
    let res = OutParseError {
        message: err.to_string(),
        line: pos.line().map(|x| x as u32),
        column: pos.position().map(|x| x as u32),
    };
    JsValue::from_serde(&res).unwrap()
}
