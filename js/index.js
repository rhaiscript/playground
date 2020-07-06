const initImport = import("./init.js");
const buefyCssImport = import("buefy/dist/buefy.css");

let embedWaitPromise;
if (window.location.hash.startsWith("#embed-") && window.parent !== window) {
    const id = window.location.hash.substr(7);
    let embedResolve;
    embedWaitPromise = new Promise((resolve, _reject) => {
        embedResolve = resolve;
    });
    function onMessage(ev) {
        if (ev.data.to === "rhai-playground" && ev.data.req === "embed-init") {
            if (typeof ev.data.code !== "string") {
                throw "Code is not a string";
            }
            embedResolve({ code: ev.data.code });
        }
    }
    window.addEventListener("message", onMessage);
    window.parent.postMessage({ from: "rhai-playground", req: "embed-loaded", id }, "*");
    const loading = document.createElement("div");
    loading.innerText = "(embedded)";
    document.getElementById("loading").appendChild(loading);
} else {
    embedWaitPromise = Promise.resolve(null);
}

Promise.all([initImport, embedWaitPromise, buefyCssImport]).then(([m, embedInit, _buefyCss]) => {
    document.getElementById("loading").remove();
    m.default("#topContainer", embedInit,)
});
