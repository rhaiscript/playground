<template>
    <div></div>
</template>

<script>
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

function initEditor(vm) {
    const editor = CodeMirror(vm.$el, {
        mode: "rhai",
        lineNumbers: true,
        indentUnit: 4,
        matchBrackets: true,
        foldGutter: {
            rangeFinder: CodeMirror.fold.brace,
        },
        gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
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
                vm.requestRun();
            },
            "Ctrl-/": "toggleComment",
        },
    });

    editor.on("change", (editor, changes) => {
        vm.change(editor, changes);
    });

    return editor;
}

export default {
    methods: {
        change(editor, changes) {
            this.$emit("change", editor, changes);
        },
        requestRun() {
            this.$emit("requestRun", this.$_cm);
        },
        getEditor() {
            return this.$_cm;
        },
    },
    mounted() {
        this.$_cm = initEditor(this);
    },
};
</script>
