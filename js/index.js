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

import("./playground.vue").catch(console.error).then(module => {
    document.getElementById("loading").remove();

    new Vue({
        el: "#topContainer",
        render(h) {
            return h(module.default);
        },
    })
})
