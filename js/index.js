import("../pkg/index.js").catch(console.error).then(
    module => {
        window.run_script = module.run_script;
    }
);
import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/mode/rust/rust.js";

const initialCode = `\
fn run(a) {
    let b = a + 1;
    print("Hello world! a = " + a);
}
run(10);
`;

const editor = CodeMirror(document.getElementById('editorContainer'), {
    value: initialCode,
    mode: "rust",
    lineNumbers: true,
    indentUnit: 4,
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
        let result = window.run_script(script, s => {
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
