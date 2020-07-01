<style scoped>
.playgroundTop {
    height: 100%;
    max-height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: auto auto 1fr 1fr;
}
@media screen and (min-width: 900px) {
    .playgroundTop {
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto auto 1fr;
    }
    .header {
        grid-column-start: 1;
        grid-column-end: 3;
    }
    .runHeader {
        grid-column-start: 1;
        grid-column-end: 3;
    }
}
.result {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    margin: 0;
    resize: none;
}
</style>

<style>
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
            <select v-model="selectedExampleScript" :disabled="exampleScriptChangePromise !== null">
                <option value>(example scripts...)</option>
                <option v-for="i in exampleScriptList" :key="i.value" :value="i.value">{{ i.text }}</option>
            </select>
            <label style="display: inline-block;">
                Theme:
                <select v-model="selectedCmTheme" :disabled="cmThemeChangePromise !== null">
                    <option value="default">Default</option>
                    <option v-for="i in cmThemeList" :key="i.value" :value="i.value">{{ i.text }}</option>
                </select>
            </label>
            <label style="display: inline-block;">
                <input type="checkbox" v-model="isRunScriptOnWorker" />
                Run script asynchronously on Web Worker
            </label>
        </header>
        <header class="runHeader">
            <!--prettyhtml-preserve-whitespace-->
            <button type="button" @click="requestRun" :disabled="runDisabled">
                Run script (<kbd>Ctrl</kbd>+<kbd>Enter</kbd>)
            </button>
            <button type="button" @click="stopScript" :disabled="stopDisabled">Stop</button>
            <span v-show="!isScriptRunning">Idle</span>
            <span v-show="isScriptRunning">Running...</span>
            /
            <label>
                Ops:
                <input type="text" :value="runningOpsDisplay" readonly style="width: 100px;" />
            </label>
            <span style="font-size: 0.8em;">
                <a href="https://github.com/alvinhochun/rhai-playground" target="_blank">rhai-playground</a>
                {{ VERSION }}
            </span>
        </header>
        <editor
            style="overflow: hidden;"
            ref="editor"
            @change="codeChange"
            @requestRun="requestRun"
        ></editor>
        <div>
            <textarea ref="result" class="result" readonly autocomplete="off"></textarea>
        </div>
    </div>
</template>

<script>
import * as wasm from "../pkg/index.js";

import Editor from "./components/editor.vue";
import * as Runner from "./playground-runner";

import CodeMirror from "codemirror";

wasm.init_codemirror_pass(CodeMirror.Pass);

CodeMirror.defineMode("rhai", (cfg, mode) => {
    return new wasm.RhaiMode(cfg.indentUnit);
});

const initialCode = `\
fn run(a) {
    let b = a + 1;
    print("Hello world! a = " + a);
}
run(10);
`;

function initEditor() {
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
            this.timeout = window.setTimeout(
                () => this._fire(arg),
                this.delayMsec
            );
        },
        _fire(editor) {
            tryCompileScript(editor);
        },
    };

    function doRunScriptSync(editor, resultEl) {
        let script = editor.getValue();
        resultEl.value = `Running script at ${new Date().toISOString()}\n\n`;
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                try {
                    let result = wasm.run_script(
                        script,
                        s => {
                            resultEl.value += `[PRINT] ${s}\n`;
                        },
                        s => {
                            resultEl.value += `[DEBUG] ${s}\n`;
                        },
                    );
                    resultEl.value += `\nScript returned: "${result}"`;
                } catch (ex) {
                    resultEl.value += `\nEXCEPTION: "${ex}"`;
                }
                resultEl.value += `\nFinished at ${new Date().toISOString()}`;
                // Scroll to bottom
                resultEl.scrollTop = resultEl.scrollHeight - resultEl.clientHeight;
                resolve();
            }, 10);
        });
    }

    let runScriptPromise = null;
    async function doRunScriptAsync(editor, el, updateOps) {
        if (runScriptPromise) {
            console.log(
                "Blocked run script request as another script is already running."
            );
            return;
        }
        let script = editor.getValue();
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
        try {
            await (runScriptPromise = Runner.runScript(script, appendOutput, updateOps));
        } catch (ex) {
            appendOutput(`\nEXCEPTION: "${ex}"`);
        } finally {
            runScriptPromise = null;
        }
    }

    let isScriptRunning = false;
    async function doRunScript(editor, isAsync, resultEl, updateOps) {
        if (isScriptRunning) {
            console.log(
                "Blocked run script request as another script is already running."
            );
            return;
        }
        isScriptRunning = true;
        if (isAsync) {
            await doRunScriptAsync(editor, resultEl, updateOps);
        } else {
            await doRunScriptSync(editor, resultEl);
        }
        isScriptRunning = false;
    }

    return {
        tryCompileDebounced,
        doRunScript,
    };
}

// With the help of webpack, we can get a list of all the example script files
// and the ability to lazily load them on demand:
const exampleScriptsImport = require.context("!raw-loader!../example-scripts/", false, /\.rhai$/, "lazy");
let exampleScriptList = [];
for (let key of exampleScriptsImport.keys()) {
    const value = key;
    if (key.startsWith("./")) {
        key = key.substr(2);
    }
    const text = key;
    exampleScriptList.push({ value, text });
}
Object.freeze(exampleScriptList);

// Include all the CodeMirror themes but load lazily:
const cmThemesImport = require.context("codemirror/theme/", false, /\.css$/, "lazy");
let cmThemeList = [];
for (let key of cmThemesImport.keys()) {
    if (!key.startsWith("./") || !key.endsWith(".css")) {
        continue;
    }
    key = key.substring(2, key.length - 4);
    function addOpt(key, name) {
        const value = name ? `${key}/${name}` : key;
        const text = name || key;
        cmThemeList.push({ value, text });
    }
    if (key === "solarized") {
        addOpt(key, `${key} dark`);
        addOpt(key, `${key} light`);
    } else {
        addOpt(key);
    }
}
Object.freeze(cmThemeList);

export default {
    data() {
        return {
            selectedExampleScript: "",
            exampleScriptList,
            exampleScriptChangePromise: null,
            selectedCmTheme: "default",
            cmThemeList,
            cmThemeChangePromise: null,
            isRunScriptOnWorker: true,
            isScriptRunning: false,
            runningOps: null,
            stopDisabled: true,
        };
    },
    computed: {
        runDisabled() {
            return this.isScriptRunning || this.exampleScriptChangePromise !== null;
        },
        runningOpsDisplay() {
            if (this.runningOps !== null) {
                return this.runningOps.toLocaleString();
            } else {
                return "-";
            }
        },
        VERSION() {
            return VERSION;
        },
    },
    methods: {
        codeChange(editor, changes) {
            this.$_r.tryCompileDebounced.trigger(editor);
        },
        async requestRun() {
            if (this.runDisabled) {
                return;
            }
            this.isScriptRunning = true;
            if (this.isRunScriptOnWorker) {
                this.stopDisabled = false;
            }
            this.runningOps = null;
            await this.$_r.doRunScript(
                this.$refs.editor.getEditor(),
                this.isRunScriptOnWorker,
                this.$refs.result,
                ops => {
                    this.runningOps = ops;
                },
            );
            this.stopDisabled = true;
            this.isScriptRunning = false;
        },
        /**
         * @returns {CodeMirror.Editor}
         */
        getEditor() {
            return this.$refs.editor.getEditor();
        },
        stopScript() {
            Runner.stopScript();
        },
    },
    watch: {
        selectedExampleScript(newVal, oldVal) {
            if (!newVal) {
                return;
            }
            const cm = this.getEditor();
            this.$_r.tryCompileDebounced.cancel();
            cm.setOption("readOnly", true);
            this.exampleScriptChangePromise = exampleScriptsImport(this.selectedExampleScript)
                .then(module => {
                    cm.setValue(module.default);
                })
                .catch(e => {
                    console.error("Error loading script", e);
                })
                .finally(() => {
                    cm.setOption("readOnly", false);
                    this.exampleScriptChangePromise = null;
                });
        },
        selectedCmTheme(theme, oldVal) {
            if (!theme) {
                return;
            }
            const cm = this.getEditor();
            if (theme === "default") {
                cm.setOption("theme", "default");
                return;
            }
            let themeFile = theme;
            const slash = theme.indexOf("/");
            if (slash !== -1) {
                themeFile = theme.substring(0, slash);
                theme = theme.substring(slash + 1);
            }
            this.cmThemeChangePromise = cmThemesImport(`./${themeFile}.css`)
                .then(module => {
                    cm.setOption("theme", theme);
                })
                .catch(e => {
                    console.error("Error loading theme", e);
                })
                .finally(() => {
                    this.cmThemeChangePromise = null;
                });
        }
    },
    mounted() {
        const cm = this.getEditor();
        const r = initEditor();
        r.tryCompileDebounced.trigger(cm);
        this.$_r = r;
        cm.setValue(initialCode);
        cm.focus();
    },
    components: { Editor },
};
</script>
