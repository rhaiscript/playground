import("../pkg/index.js").catch(console.error);
import * as monaco from 'monaco-editor';

monaco.editor.create(document.getElementById('editorContainer'), {
    value: 'fn run(a) {\n    let b = a + 1;\n    print("Hello world! a = " + a);\n}\nrun(10);\n',
    language: 'rust'
});
