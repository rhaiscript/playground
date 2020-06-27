// FIXME: We could have used worker-loader but it currently breaks with WASM.
//        workerize-loader has the issue fixed. We sould want to switch back
//        to worker-loader once the issue is fixed.
//        Blocked on: https://github.com/webpack-contrib/worker-loader/pull/175
import MyWorker from "workerize-loader!./worker.js";

/**
 * @type Worker
 */
const worker = new MyWorker();

worker.onerror = ev => {
    console.error("An error occured in the worker:", ev);
};

let runScriptMessageListener = null;

/**
 * @callback AppendOutputCallback
 * @param {string} line
 */

/**
 * 
 * @param {string} script
 * @param {AppendOutputCallback} appendOutput
 * @returns {Promise<void>}
 */
function runScript(script, appendOutput) {
    if (runScriptMessageListener) {
        return Promise.reject("Another script is running.");
    }
    return new Promise((resolve, _reject) => {
        appendOutput(`Running script at ${new Date().toISOString()}\n`);
        worker.addEventListener("message", runScriptMessageListener = ev => {
            if (ev.data.req === "runScript/output") {
                appendOutput(ev.data.output);
            } else if (ev.data.req === "runScript/end") {
                appendOutput(`Finished at ${new Date().toISOString()}`);
                worker.removeEventListener("message", runScriptMessageListener);
                runScriptMessageListener = null;
                resolve();
            }
        })
        worker.postMessage({ req: "runScript", script });
    });
}

export { runScript };
