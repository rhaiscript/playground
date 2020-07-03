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
.header {
    padding: 0.75rem;
}
.outputPanel {
    display: flex;
    flex-direction: column;
}
.result {
    box-sizing: border-box;
    margin: 0;
    resize: none;
    font-family: monospace;
    flex-grow: 1;
}
</style>

<style>
.CodeMirror {
    border: 1px solid #ccc;
    height: 100% !important;
    box-sizing: border-box;
    font-size: 0.95em;
    line-height: initial;
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
            <b-field grouped group-multiline>
                <b-field>
                    <p class="control">
                        <b-button
                            type="is-success"
                            native-type="button"
                            icon-left="play"
                            @click="requestRun"
                            :disabled="runDisabled"
                        >Run</b-button>
                    </p>
                    <p class="control" v-if="isRunScriptOnWorker">
                        <b-tooltip
                            position="is-bottom"
                            :label="(isScriptRunning ? 'Running' : 'Idle') + (runningOps ? ` / Ops: ${runningOpsDisplay}` : '')"
                            :always="isScriptRunning && runningOps !== null"
                        >
                            <b-button
                                type="is-danger"
                                native-type="button"
                                icon-left="stop"
                                @click="stopScript"
                                :disabled="stopDisabled"
                            >Stop</b-button>
                        </b-tooltip>
                    </p>
                </b-field>
                <b-field style="margin-bottom: 0.75rem;">
                    <p class="control">
                        <b-dropdown
                            aria-role="menu"
                            :disabled="exampleScriptChangePromise !== null || isScriptRunning"
                        >
                            <button
                                class="button"
                                position="is-bottom-left"
                                slot="trigger"
                                role="button"
                                type="button"
                            >
                                <span>Example Scripts</span>
                                <b-icon icon="menu-down" />
                            </button>
                            <b-dropdown-item
                                aria-role="menu-item"
                                v-for="i in exampleScriptList"
                                :key="i.value"
                                @click.native.prevent="loadExampleScript(i.value)"
                                href="#"
                            >{{ i.text }}</b-dropdown-item>
                        </b-dropdown>
                    </p>
                    <p class="control">
                        <b-dropdown aria-role="menu">
                            <b-button
                                icon-left="cog"
                                slot="trigger"
                                role="button"
                                native-type="button"
                            >Config</b-button>

                            <b-dropdown-item aria-role="menu-item" :focusable="false" custom>
                                <b-field label="Theme">
                                    <b-select
                                        v-model="selectedCmTheme"
                                        :disabled="cmThemeChangePromise !== null"
                                    >
                                        <option value="default">Default</option>
                                        <option
                                            v-for="i in cmThemeList"
                                            :key="i.value"
                                            :value="i.value"
                                        >{{ i.text }}</option>
                                    </b-select>
                                </b-field>
                                <div class="field">
                                    <b-switch
                                        v-model="isRunScriptOnWorker"
                                        :disabled="isScriptRunning"
                                    >
                                        Run script using
                                        <b>Web Worker</b>
                                    </b-switch>
                                </div>
                            </b-dropdown-item>
                        </b-dropdown>
                    </p>
                    <p class="control">
                        <b-dropdown aria-role="menu">
                            <b-button
                                icon-left="help-circle"
                                slot="trigger"
                                role="button"
                                native-type="button"
                            ></b-button>

                            <b-dropdown-item
                                aria-role="menu-item"
                                :focusable="false"
                                custom
                                paddingless
                            >
                                <div class="modal-card" style="width: 300px;">
                                    <section class="modal-card-body">
                                        <div class="content">
                                            <h1>What is Rhai?</h1>
                                            <p>
                                                <a
                                                    href="https://github.com/jonathandturner/rhai"
                                                    target="_blank"
                                                >Rhai</a> is an embedded scripting language and evaluation engine for Rust.
                                            </p>
                                            <h1>Hotkeys</h1>
                                            <p>
                                                You can run the script by pressing
                                                <kbd>Ctrl</kbd> +
                                                <kbd>Enter</kbd> when focused in the editor.
                                            </p>
                                        </div>
                                    </section>
                                    <footer class="modal-card-foot">
                                        <span>
                                            <a
                                                href="https://github.com/alvinhochun/rhai-playground"
                                                target="_blank"
                                            >rhai-playground</a>
                                            version: {{ VERSION }}
                                        </span>
                                    </footer>
                                </div>
                            </b-dropdown-item>
                        </b-dropdown>
                    </p>
                </b-field>
            </b-field>
        </header>
        <editor
            style="overflow: hidden;"
            ref="editor"
            @change="codeChange"
            @requestRun="requestRun"
        ></editor>
        <div class="outputPanel">
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
        loadExampleScript(key) {
            const cm = this.getEditor();
            this.$_r.tryCompileDebounced.cancel();
            cm.setOption("readOnly", true);
            this.exampleScriptChangePromise = exampleScriptsImport(key)
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
    },
    watch: {
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
        },
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
