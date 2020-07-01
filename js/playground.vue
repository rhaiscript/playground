<style scoped>
.playgroundTop {
    height: 100%;
    max-height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: auto 1fr 1fr;
}
@media screen and (min-width: 900px) {
    .playgroundTop {
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto 1fr;
    }
    .header {
        grid-column-start: 1;
        grid-column-end: 3;
    }
}
#editorContainer {
    overflow: hidden;
}
#result {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    margin: 0;
    resize: none;
}
.CodeMirror {
    border: 1px solid #ccc;
    height: 100% !important;
    box-sizing: border-box;
    font-size: 11pt;
}
.CodeMirror .rhai-error {
    text-decoration: underline wavy red;
}
.CodeMirror .cm-matchhighlight {
    background-color: rgba(0, 0, 0, 0.1);
}
.CodeMirror .CodeMirror-selection-highlight-scrollbar {
    background-color: rgba(0, 0, 0, 0.1);
}
</style>

<template>
    <div class="playgroundTop">
        <header class="header">
            <button type="button" id="runScriptButton">
                Run script (
                <kbd>Ctrl</kbd>+
                <kbd>Enter</kbd>)
            </button>
            <select id="exampleScriptSelect">
                <option value>(example scripts...)</option>
            </select>
            <label style="display: inline-block;">
                Theme:
                <select id="cmThemeSelect">
                    <option value="default">Default</option>
                </select>
            </label>
            <label style="display: inline-block;">
                <input id="runScriptOnWorker" type="checkbox" checked />
                Run script asynchronously on Web Worker
            </label>
        </header>
        <div id="editorContainer" style="overflow: hidden;"></div>
        <div id="outputContainer">
            <textarea
                readonly
                id="result"
                style="width: 100%; height: 100%; box-sizing: border-box; margin: 0;"
                autocomplete="off"
            ></textarea>
        </div>
    </div>
</template>

<script>
import * as wasm from "../pkg/index.js";

import { runScript } from "./playground-runner";

import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/addon/comment/comment";
import "codemirror/addon/display/rulers";
import "codemirror/addon/edit/closebrackets";
import "codemirror/addon/edit/matchbrackets";
import "codemirror/addon/fold/brace-fold";
import "codemirror/addon/fold/foldgutter";
import "codemirror/addon/fold/foldgutter.css";
import "codemirror/addon/search/match-highlighter";
import "codemirror/addon/selection/active-line";

wasm.init_codemirror_pass(CodeMirror.Pass);

const initialCode = `\
fn run(a) {
    let b = a + 1;
    print("Hello world! a = " + a);
}
run(10);
`;

function initEditor() {
    // With the help of webpack, we can get a list of all the example script files
    // and the ability to lazily load them on demand:
    const exampleScriptsImport = require.context(
        "!raw-loader!../example-scripts/",
        false,
        /\.rhai$/,
        "lazy"
    );
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

    // Include all the CodeMirror themes but load lazily:
    const cmThemesImport = require.context(
        "codemirror/theme/",
        false,
        /\.css$/,
        "lazy"
    );
    const cmThemeSelect = document.getElementById("cmThemeSelect");
    for (let key of cmThemesImport.keys()) {
        if (!key.startsWith("./") || !key.endsWith(".css")) {
            continue;
        }
        key = key.substring(2, key.length - 4);
        function addOpt(key, name) {
            const opt = document.createElement("option");
            opt.value = name ? `${key}/${name}` : key;
            opt.innerText = name || key;
            cmThemeSelect.appendChild(opt);
        }
        if (key === "solarized") {
            addOpt(key, `${key} dark`);
            addOpt(key, `${key} light`);
        } else {
            addOpt(key);
        }
    }

    CodeMirror.defineMode("rhai", (cfg, mode) => {
        return new wasm.RhaiMode(cfg.indentUnit);
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
            wasm.compile_script(editor.getValue());
        } catch (e) {
            console.log("Parse error:", e);
            if (typeof e.message === "string" && e.line && e.column) {
                lastErrorMarker = editor.markText(
                    { line: e.line - 1, ch: e.column - 1 },
                    { line: e.line - 1, ch: e.column },
                    {
                        className: "rhai-error",
                        title: e.message
                    }
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
            this.timeout = window.setTimeout(
                () => this._fire(arg),
                this.delayMsec
            );
        },
        _fire(editor) {
            tryCompileScript(editor);
        }
    };

    const runScriptButton = document.getElementById("runScriptButton");

    function doRunScriptSync(editor) {
        let script = editor.getValue();
        let resultEl = document.getElementById("result");
        resultEl.value = `Running script at ${new Date().toISOString()}\n\n`;
        runScriptButton.disabled = true;
        setTimeout(() => {
            try {
                let result = wasm.run_script(
                    script,
                    s => {
                        resultEl.value += `[PRINT] ${s}\n`;
                    },
                    s => {
                        resultEl.value += `[DEBUG] ${s}\n`;
                    }
                );
                resultEl.value += `\nScript returned: "${result}"`;
            } catch (ex) {
                resultEl.value += `\nEXCEPTION: "${ex}"`;
            }
            resultEl.value += `\nFinished at ${new Date().toISOString()}`;
            // Scroll to bottom
            resultEl.scrollTop = resultEl.scrollHeight - resultEl.clientHeight;
            runScriptButton.disabled = false;
        }, 10);
    }

    let runScriptPromise = null;
    async function doRunScriptAsync(editor) {
        if (runScriptPromise) {
            console.log(
                "Blocked run script request as another script is already running."
            );
            return;
        }
        let script = editor.getValue();
        let el = document.getElementById("result");
        el.value = "";
        function appendOutput(line) {
            // FIXME: The auto-scroll code causes a lot of extra layout events,
            //        which drastically slows down if the script prints a lot of
            //        lines. Can it be made less bad?
            // There may be a 1px offset on certain scaling conditions so we give
            // it a bit of leeway.
            const scroll =
                el.scrollTop >= el.scrollHeight - el.clientHeight - 2;
            el.value += line + "\n";
            if (scroll) {
                // This amount should be large enough.
                el.scrollTop += 1000;
            }
        }
        runScriptButton.disabled = true;
        try {
            await (runScriptPromise = runScript(script, appendOutput));
        } finally {
            runScriptButton.disabled = false;
            runScriptPromise = null;
        }
    }

    const runScriptOnWorkerCheckbox = document.getElementById(
        "runScriptOnWorker"
    );
    let isScriptRunning = false;
    function doRunScript(editor) {
        if (isScriptRunning) {
            console.log(
                "Blocked run script request as another script is already running."
            );
            return;
        }
        isScriptRunning = true;
        if (runScriptOnWorkerCheckbox.checked) {
            doRunScriptAsync(editor).then(() => {
                isScriptRunning = false;
            });
        } else {
            doRunScriptSync(editor);
            isScriptRunning = false;
        }
    }

    const editor = CodeMirror(document.getElementById("editorContainer"), {
        value: initialCode,
        mode: "rhai",
        lineNumbers: true,
        indentUnit: 4,
        matchBrackets: true,
        foldGutter: {
            rangeFinder: CodeMirror.fold.brace
        },
        gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
        styleActiveLine: true,
        highlightSelectionMatches: {
            minChars: 3,
            showToken: true,
            annotateScrollbar: true
        },
        rulers: [
            {
                column: 80,
                color: "#ccc"
            }
        ],
        autoCloseBrackets: {
            pairs: `()[]{}''""`,
            closeBefore: `)]}'":;,`,
            triples: "",
            explode: "()[]{}"
        },
        extraKeys: {
            Tab: cm => {
                // This function is a modification of `defaultTab` to insert soft
                // tab instead of hard tab.
                if (cm.somethingSelected()) {
                    cm.indentSelection("add");
                } else {
                    cm.execCommand("insertSoftTab");
                }
            },
            "Ctrl-Enter": cm => {
                doRunScript(cm);
            },
            "Ctrl-/": "toggleComment"
        }
    });

    editor.on("change", (editor, changes) => {
        tryCompileDebounced.trigger(editor);
    });

    tryCompileDebounced.trigger(editor);

    runScriptButton.addEventListener("click", ev => {
        doRunScript(editor);
    });

    exampleScriptSelect.addEventListener("change", ev => {
        if (!exampleScriptSelect.value) {
            return;
        }
        tryCompileDebounced.cancel();
        editor.setOption("readOnly", true);
        exampleScriptsImport(exampleScriptSelect.value)
            .then(module => {
                editor.setValue(module.default);
                editor.setOption("readOnly", false);
            })
            .catch(e => {
                console.error("Error loading script", e);
                editor.setOption("readOnly", false);
            });
    });

    cmThemeSelect.addEventListener("change", ev => {
        let theme = cmThemeSelect.value;
        let themeFile = theme;
        if (!theme) {
            return;
        }
        if (theme === "default") {
            editor.setOption("theme", "default");
            return;
        }
        const slash = theme.indexOf("/");
        if (slash !== -1) {
            themeFile = theme.substring(0, slash);
            theme = theme.substring(slash + 1);
        }
        cmThemeSelect.disabled = true;
        cmThemesImport(`./${themeFile}.css`)
            .then(module => {
                editor.setOption("theme", theme);
            })
            .catch(e => {
                console.error("Error loading theme", e);
            })
            .finally(() => {
                cmThemeSelect.disabled = false;
            });
    });
}

export default {
    data() {
        return {};
    },
    mounted() {
        initEditor();
    }
};
</script>
