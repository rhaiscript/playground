const wasmImport = import("../pkg/index.js").catch(console.error);

import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/addon/edit/closebrackets"

const initialCode = `\
fn run(a) {
    let b = a + 1;
    print("Hello world! a = " + a);
}
run(10);
`;

wasmImport.then(module => {
    CodeMirror.defineMode("rhai", (cfg, mode) => {
        return new module.RhaiMode(cfg.indentUnit);
    });

    const editor = CodeMirror(document.getElementById('editorContainer'), {
        value: initialCode,
        mode: "rhai",
        lineNumbers: true,
        indentUnit: 4,
        autoCloseBrackets: {
            pairs: `()[]{}''""`,
            closeBefore: `)]}'":;,`,
            triples: "",
            explode: "()[]{}",
        },
        extraKeys: {
            "Tab": cm => {
                // This function is a modification of `defaultTab` to insert soft
                // tab instead of hard tab.
                if (cm.somethingSelected()) {
                    cm.indentSelection("add");
                } else {
                    cm.execCommand("insertSoftTab");
                }
            },
            "Ctrl-Enter": cm => {
                doRunScript();
            },
        },
    });

    function doRunScript() {
        let resultEl = document.getElementById('result');
        resultEl.value = `Running script at ${new Date().toISOString()}\n\n`;
        try {
            let script = editor.getValue();
            let result = module.run_script(script, s => {
                resultEl.value += `[PRINT] ${s}\n`;
            }, s => {
                resultEl.value += `[DEBUG] ${s}\n`;
            });
            resultEl.value += `\nScript returned: "${result}"`;
        } catch (ex) {
            resultEl.value += `\nEXCEPTION: "${ex}"`;
        }
        resultEl.value += `\nFinished at ${new Date().toISOString()}`;
    }
    window.btnClick = doRunScript;
});
