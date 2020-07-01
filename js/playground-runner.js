// FIXME: We could have used worker-loader but it currently breaks with WASM.
//        workerize-loader has the issue fixed. We sould want to switch back
//        to worker-loader once the issue is fixed.
//        Blocked on: https://github.com/webpack-contrib/worker-loader/pull/175
import MyWorker from "workerize-loader!./worker.js";

/**
 * @type Worker?
 */
let worker = null;

function ensureWorker() {
    if (worker === null) {
        worker = new MyWorker();
        worker.onerror = ev => {
            console.error("An error occured in the worker:", ev);
        };
    }
    return worker;
}

function terminateWorker() {
    if (worker != null) {
        worker.terminate();
        worker = null;
    }
}

let runScriptMessageListener = null;
let runScriptPromiseReject = null;

/**
 * @callback AppendOutputCallback
 * @param {string} line
 */

/**
 * 
 * @param {string} script
 * @param {AppendOutputCallback} appendOutput
 * @param {(Number) => void} updateOps
 * @returns {Promise<void>}
 */
function runScript(script, appendOutput, updateOps) {
    ensureWorker();
    if (runScriptMessageListener) {
        return Promise.reject("Another script is running.");
    }
    return new Promise((resolve, reject) => {
        appendOutput(`Running script at ${new Date().toISOString()}\n`);
        worker.addEventListener("message", runScriptMessageListener = ev => {
            if (ev.data.req === "runScript/output") {
                appendOutput(ev.data.output);
            } else if (ev.data.req === "runScript/end") {
                appendOutput(`Finished at ${new Date().toISOString()}`);
                worker.removeEventListener("message", runScriptMessageListener);
                runScriptMessageListener = null;
                runScriptPromiseReject = null;
                resolve();
            } else if (ev.data.req === "runScript/updateOps") {
                updateOps(ev.data.ops);
            }
        })
        runScriptPromiseReject = reject;
        worker.postMessage({ req: "runScript", script });
    });
}

function stopScript() {
    if (runScriptPromiseReject) {
        terminateWorker();
        runScriptPromiseReject("Script execution stopped.");
        runScriptMessageListener = null;
        runScriptPromiseReject = null;
    }
}

export { runScript, stopScript };
