use rhai::packages::{EvalPackage, Package};

pub fn run_script(
    script: &str,
    print_callback: impl Fn(&str) + 'static,
    debug_callback: impl Fn(&str) + 'static,
) -> Result<String, String> {
    let engine = {
        let mut engine = rhai::Engine::new();
        engine.load_package(EvalPackage::new().get());
        engine.on_print(move |s| print_callback(s));
        engine.on_debug(move |s| debug_callback(s));
        engine
    };
    let script_ast = engine.compile(&script).map_err(|e| e.to_string())?;

    let result: rhai::Dynamic = engine.eval_ast(&script_ast).map_err(|e| e.to_string())?;
    Ok(result.to_string())
}
