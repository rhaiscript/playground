use instant::Instant;
use rhai::{
    packages::{EvalPackage, Package},
    Engine, EvalAltResult, Module, ModuleResolver, Scope,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;

pub struct Playground {
    engine: Engine,
    modules: HashMap<Box<str>, Module>,
}

impl Playground {
    pub fn new() -> Self {
        let mut engine = rhai::Engine::new();
        engine.load_package(EvalPackage::new().get());
        engine.on_print(|_| {});
        engine.on_debug(|_| {});
        Self {
            engine: Engine::new(),
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, name: String, script: &str) -> Result<String, String> {
        let engine = &mut self.engine;
        // TODO: Hook print/debug/progress
        let script_ast = engine.compile(&script).map_err(|e| e.to_string())?;
        let module = Module::eval_ast_as_new(Scope::new(), &script_ast, &*engine)
            .map_err(|e| e.to_string())?;
        self.modules.insert(name.into_boxed_str(), module);
        Ok("".into())
    }

    pub fn run_script(
        &mut self,
        script: &str,
        print_callback: impl Fn(&str) + 'static,
        debug_callback: impl Fn(&str) + 'static,
        progress_callback: impl Fn(u64) + 'static,
    ) -> Result<String, String> {
        struct Defer<'z> {
            mut_self: &'z mut Playground,
            modules_rc: Option<Rc<RefCell<HashMap<Box<str>, Module>>>>,
        }

        // Take the list of modules and put it in an Rc<RefCell<_>> because
        // `set_module_resolver` requires `'static` lifetime.
        let modules_rc = Rc::new(RefCell::new(std::mem::replace(
            &mut self.modules,
            HashMap::new(),
        )));
        let defer = Defer {
            mut_self: self,
            modules_rc: Some(Rc::clone(&modules_rc)),
        };
        let engine = &mut defer.mut_self.engine;

        engine.on_print(move |s| print_callback(s));
        engine.on_debug(move |s| debug_callback(s));
        let script_ast = engine.compile(&script).map_err(|e| e.to_string())?;

        let interval = RefCell::new(1000);
        let last_instant = RefCell::new(Instant::now());
        engine.on_progress(move |&ops| {
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
            true
        });

        let module_stor = ModuleStorage(modules_rc);
        engine.set_module_resolver(Some(module_stor));

        let result: rhai::Dynamic = engine.eval_ast(&script_ast).map_err(|e| e.to_string())?;

        return Ok(result.to_string());

        impl Drop for Defer<'_> {
            fn drop(&mut self) {
                let engine = &mut self.mut_self.engine;
                engine.on_print(|_| {});
                engine.on_debug(|_| {});
                engine.on_progress(|_| true);
                engine.set_module_resolver(None as Option<ModuleStorage>);
                // The `Engine` should no longer be holding a reference.
                let modules = Rc::try_unwrap(self.modules_rc.take().unwrap())
                    .expect("Failed to unwrap the `Rc`")
                    .into_inner();
                // Now we can put it back:
                self.mut_self.modules = modules;
            }
        }
    }
}

struct ModuleStorage(Rc<RefCell<HashMap<Box<str>, Module>>>);

impl ModuleResolver for ModuleStorage {
    fn resolve(
        &self,
        _: &Engine,
        path: &str,
        pos: rhai::Position,
    ) -> Result<Module, Box<rhai::EvalAltResult>> {
        self.0
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| Box::new(EvalAltResult::ErrorModuleNotFound(path.into(), pos)))
    }
}

#[wasm_bindgen(js_name = Playground)]
pub struct PlaygroundExport(Playground);

#[wasm_bindgen(js_class = Playground)]
impl PlaygroundExport {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Playground::new())
    }

    #[wasm_bindgen(js_name = addModule)]
    pub fn add_module(&mut self, name: String, script: &str) -> Result<String, JsValue> {
        Ok(self.0.add_module(name, script)?)
    }

    #[wasm_bindgen(js_name = runScript)]
    pub fn run_script(
        &mut self,
        script: String,
        print_callback: js_sys::Function,
        debug_callback: js_sys::Function,
        progress_callback: Option<js_sys::Function>,
    ) -> Result<String, JsValue> {
        Ok(self.0.run_script(
            &script,
            move |s| {
                let _ = print_callback.call1(&JsValue::null(), &JsValue::from_str(s));
            },
            move |s| {
                let _ = debug_callback.call1(&JsValue::null(), &JsValue::from_str(s));
            },
            move |ops| {
                if let Some(f) = &progress_callback {
                    let _ = f.call1(&JsValue::null(), &JsValue::from_f64(ops as f64));
                }
            },
        )?)
    }
}
