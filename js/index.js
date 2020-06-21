import("../pkg/index.js").catch(console.error).then(
    module => {
        window.run_script = module.run_script;
    }
);
import * as monaco from 'monaco-editor';

let editor = monaco.editor.create(document.getElementById('editorContainer'), {
    value: 'fn run(a) {\n    let b = a + 1;\n    print("Hello world! a = " + a);\n}\nrun(10);\n',
    language: 'rust',
    automaticLayout: true,
});

function btnClick() {
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
window.btnClick = btnClick;
