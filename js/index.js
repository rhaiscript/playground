import Vue from "vue";
import { Button, Dropdown, Field, Icon, Select, Switch, Tooltip } from "buefy";

import "buefy/dist/buefy.css";

Vue.use(Button);
Vue.use(Dropdown);
Vue.use(Field);
Vue.use(Icon);
Vue.use(Select);
Vue.use(Switch);
Vue.use(Tooltip);

const playgroundImport = import("./playground.vue").catch(console.error);

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

Promise.all([playgroundImport, embedWaitPromise]).then(([module, embedInit]) => {
    document.getElementById("loading").remove();

    const playground = module.default;
    const initialCode = embedInit ? embedInit.code : undefined;
    const isEmbedded = embedInit ? true : false;

    new Vue({
        el: "#topContainer",
        render(h) {
            return h(playground, {
                props: {
                    initialCode,
                    isEmbedded,
                },
            });
        },
    })
})
