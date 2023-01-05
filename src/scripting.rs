use instant::Instant;
use rhai::ParseError;
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use web_sys::console;

pub fn run_script(
    script: &str,
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
    progress_callback: impl Fn(u64) + 'static,
) -> Result<String, String> {
    let mut engine = {
        let mut engine = rhai::Engine::new();
        engine
            .disable_symbol("eval")
            .on_print(move |s| print_callback(s))
            .on_debug(move |s, src, pos| {
                debug_callback(&src.map_or_else(
                    || format!("<script>:[{}] {}", pos, s),
                    |src| format!("{}:[{}] {}", src, pos, s),
                ))
            });
        engine
    };
    let script_ast = engine.compile(&script).map_err(|e| e.to_string())?;

    let interval = RefCell::new(1000);
    let last_instant = RefCell::new(Instant::now());
    engine.on_progress(move |ops| {
        let interval_value = *interval.borrow();
        if ops % interval_value == 0 {
            let mut last_instant = last_instant.borrow_mut();
            let new_instant = Instant::now();
            let duration_msec = new_instant.duration_since(*last_instant).as_millis();
            if duration_msec < 50 {
                interval.replace(interval_value * 10);
            } else if duration_msec >= 100 {
                progress_callback(ops);
                *last_instant = new_instant;
                if duration_msec >= 500 && interval_value > 1 {
                    interval.replace(interval_value / 10);
                }
            }
        }
        None
    });

    let result: rhai::Dynamic = engine.eval_ast(&script_ast).map_err(|e| e.to_string())?;
    Ok(result.to_string())
}

thread_local! {
    static ENGINE_FOR_AST_ONLY: rhai::Engine = {
        let mut engine = rhai::Engine::new();
        engine.set_optimization_level(rhai::OptimizationLevel::None);
        engine
    };
}

pub fn compile_ast(script: &str) -> Result<String, JsValue> {
    ENGINE_FOR_AST_ONLY.with(|engine| {
        let script_ast = engine.compile(&script).map_err(parse_error_to_js)?;
        console::log_1(&JsValue::from_str("Script compiled to AST!"));
        #[allow(deprecated)]
        let statements = script_ast.statements();
        #[allow(deprecated)]
        let module = script_ast.lib();
        let mut s = format!("//This is the Debug representation of the AST.\n\n// Statements:\n{:#?}\n\n// Modules (script-defined functions):\n", statements);
        for f in module.iter_script_fn_info() {
            use std::fmt::Write;
            writeln!(&mut s, "{:#?}", &f).unwrap();
        }
        Ok(s)
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
