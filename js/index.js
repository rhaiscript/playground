const wasmImport = import("../pkg/index.js").catch(console.error);

import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/addon/comment/comment"
import "codemirror/addon/display/rulers"
import "codemirror/addon/edit/closebrackets"
import "codemirror/addon/edit/matchbrackets"
import "codemirror/addon/fold/brace-fold"
import "codemirror/addon/fold/foldgutter"
import "codemirror/addon/fold/foldgutter.css"
import "codemirror/addon/search/match-highlighter"
import "codemirror/addon/selection/active-line"

const initialCode = `\
fn run(a) {
    let b = a + 1;
    print("Hello world! a = " + a);
}
run(10);
`;

// With the help of webpack, we can get a list of all the example script files
// and the ability to lazily load them on demand:
const exampleScriptsImport = require.context("!raw-loader!../example-scripts/", false, /\.rhai$/, "lazy");
const exampleScriptSelect = document.getElementById("exampleScriptSelect");
for (let key of exampleScriptsImport.keys()) {
    const opt = document.createElement("option");
    opt.value = key;
    if (key.startsWith("./")) {
        key = key.substr(2);
    }
    opt.innerText = key;
    exampleScriptSelect.appendChild(opt);
}

wasmImport.then(module => {
    CodeMirror.defineMode("rhai", (cfg, mode) => {
        return new module.RhaiMode(cfg.indentUnit);
    });

    const editor = CodeMirror(document.getElementById('editorContainer'), {
        value: initialCode,
        mode: "rhai",
        lineNumbers: true,
        indentUnit: 4,
        matchBrackets: true,
        foldGutter: {
            rangeFinder: CodeMirror.fold.brace,
        },
        gutters: [
            "CodeMirror-linenumbers",
            "CodeMirror-foldgutter",
        ],
        styleActiveLine: true,
        highlightSelectionMatches: {
            minChars: 3,
            showToken: true,
            annotateScrollbar: true,
        },
        rulers: [
            {
                column: 80,
                color: "#ccc",
            },
        ],
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
            "Ctrl-/": "toggleComment",
        },
    });

    /**
     * @type CodeMirror.TextMarker?
     */
    let lastErrorMarker = null;
    /**
     * 
     * @param {CodeMirror.Editor} editor 
     */
    function tryCompileScript(editor) {
        if (lastErrorMarker) {
            lastErrorMarker.clear();
            lastErrorMarker = null;
        }
        try {
            module.compile_script(editor.getValue());
        } catch (e) {
            console.log("Parse error:", e);
            if (typeof e.message === "string" && e.line && e.column) {
                lastErrorMarker = editor.markText(
                    { line: e.line - 1, ch: e.column - 1 },
                    { line: e.line - 1, ch: e.column },
                    {
                        className: "rhai-error",
                        title: e.message,
                    },
                );
            }
        }
    }

    const tryCompileDebounced = {
        delayMsec: 500,
        timeout: null,
        cancel() {
            if (this.timeout !== null) {
                window.clearTimeout(this.timeout);
            }
        },
        trigger(arg) {
            this.cancel();
            this.timeout = window.setTimeout(() => this._fire(arg), this.delayMsec);
        },
        _fire(editor) {
            tryCompileScript(editor)
        },
    };
    editor.on("change", (editor, changes) => {
        tryCompileDebounced.trigger(editor);
    });

    tryCompileDebounced.trigger(editor);

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

    exampleScriptSelect.addEventListener("change", ev => {
        if (!exampleScriptSelect.value) {
            return;
        }
        tryCompileDebounced.cancel();
        editor.setOption("readOnly", true);
        exampleScriptsImport(exampleScriptSelect.value).then(module => {
            editor.setValue(module.default);
            editor.setOption("readOnly", false);
        }).catch(e => {
            console.error("Error loading script", e);
            editor.setOption("readOnly", false);
        });
    });
});
