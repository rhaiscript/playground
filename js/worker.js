import { wasmImport } from "./wasm_loader.js";

const playgroundPromise = wasmImport.then(wasm => new wasm.Playground);

async function runScript(script) {
    const playground = await playgroundPromise;
    function output(line) {
        self.postMessage({
            req: "runScript/output",
            output: line,
        });
    }
    try {
        let result = playground.runScript(script, s => {
            output(`[PRINT] ${s}`);
        }, s => {
            output(`[DEBUG] ${s}`);
        }, ops => {
            self.postMessage({
                req: "runScript/updateOps",
                ops,
            });
        });
        output(`\nScript returned: "${result}"`);
    } catch (ex) {
        output(`\nEXCEPTION: ${ex}`);
    }
    postMessage({
        req: "runScript/end",
    });
}

async function addModule(name, script) {
    const playground = await playgroundPromise;
    try {
        let _result = playground.addModule(name, script);
        postMessage({
            req: "addModule/end",
        });
    } catch (ex) {
        postMessage({
            req: "addModule/end",
            err: ex,
        });
    }
}

self.onmessage = ev => {
    if (ev.data.req === "runScript") {
        runScript(ev.data.script);
    } else if (ev.data.req === "addModule") {
        addModule(ev.data.name, ev.data.script);
    } else {
        console.log("Unknown message received by worker:", ev.data);
    }
};

wasmImport.then(() => {
    self.postMessage({ req: "_ready" });
});
