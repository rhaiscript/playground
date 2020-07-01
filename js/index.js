import Vue from "vue";

import("./playground.vue").catch(console.error).then(module => {
    document.getElementById("loading").remove();

    new Vue({
        el: "#topContainer",
        render(h) {
            return h(module.default);
        },
    })
})
