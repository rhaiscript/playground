const wasmImport = import("../pkg/index.js").catch(console.error);

async function runScript(script) {
    const wasm = await wasmImport;
    function output(line) {
        self.postMessage({
            req: "runScript/output",
            output: line,
        });
    }
    try {
        let result = wasm.run_script(script, s => {
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

self.onmessage = ev => {
    if (ev.data.req === "runScript") {
        runScript(ev.data.script);
    } else {
        console.log("Unknown message received by worker:", ev.data);
    }
};

self.postMessage({ req: "_ready" });
