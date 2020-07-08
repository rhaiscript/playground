<template>
    <div style="overflow: hidden;"></div>
</template>

<script>
import CodeMirror from "codemirror";

// This function is mostly copied from `codemirror/addon/fold/brace-fold.js`,
// the code for the "brace" type "fold" helper.
function rangeFinder(cm, start) {
    var line = start.line, lineText = cm.getLine(line);
    var tokenType;

    function findOpening(openCh) {
        for (var at = start.ch, pass = 0; ;) {
            var found = at <= 0 ? -1 : lineText.lastIndexOf(openCh, at - 1);
            if (found == -1) {
                if (pass == 1) break;
                pass = 1;
                at = lineText.length;
                continue;
            }
            if (pass == 1 && found < start.ch) break;
            tokenType = cm.getTokenTypeAt(CodeMirror.Pos(line, found + 1));
            if (!/^(comment|string)/.test(tokenType)) return found + 1;
            at = found - 1;
        }
    }

    var startToken = "{", endToken = "}", startCh = findOpening("{");
    if (startCh == null) {
        startToken = "[", endToken = "]";
        startCh = findOpening("[");
    }
    // This `if` block is added to handle parentheses folding.
    if (startCh == null) {
        startToken = "(", endToken = ")";
        startCh = findOpening("(");
    }

    if (startCh == null) return;
    var count = 1, lastLine = cm.lastLine(), end, endCh;
    outer: for (var i = line; i <= lastLine; ++i) {
        var text = cm.getLine(i), pos = i == line ? startCh : 0;
        for (; ;) {
            var nextOpen = text.indexOf(startToken, pos), nextClose = text.indexOf(endToken, pos);
            if (nextOpen < 0) nextOpen = text.length;
            if (nextClose < 0) nextClose = text.length;
            pos = Math.min(nextOpen, nextClose);
            if (pos == text.length) break;
            if (cm.getTokenTypeAt(CodeMirror.Pos(i, pos + 1)) == tokenType) {
                if (pos == nextOpen)++count;
                else if (!--count) { end = i; endCh = pos; break outer; }
            }
            ++pos;
        }
    }
    if (end == null || line == end) return;
    return {        from: CodeMirror.Pos(line, startCh),
        to: CodeMirror.Pos(end, endCh)    };
}

function initEditor(vm) {
    const editor = CodeMirror(vm.$el, {
        mode: "text/plain",
        matchBrackets: true,
        foldGutter: {
            rangeFinder,
        },
        gutters: ["CodeMirror-foldgutter"],
        styleActiveLine: true,
        highlightSelectionMatches: {
            minChars: 3,
            showToken: true,
            annotateScrollbar: true,
        },
        readOnly: true,
    });


    return editor;
}

export default {
    props: {
        astText: String,
    },
    methods: {
        /**
         * @returns {CodeMirror.Editor}
         */
        getEditor() {
            return this._cm;
        },
    },
    watch: {
        astText(newVal) {
            const cm = this.getEditor();
            cm.setValue(newVal);
        },
    },
    mounted() {
        this._cm = initEditor(this);
    },
};
</script>
