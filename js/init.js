import Vue from "vue";
import { Button, Dropdown, Field, Icon, Select, Switch, Tooltip } from "buefy";

import playground from "./playground.vue";

Vue.use(Button);
Vue.use(Dropdown);
Vue.use(Field);
Vue.use(Icon);
Vue.use(Select);
Vue.use(Switch);
Vue.use(Tooltip);

export default function (el, embedInit) {
    const initialCode = embedInit ? embedInit.code : undefined;
    const isEmbedded = embedInit ? true : false;

    new Vue({
        el,
        render(h) {
            return h(playground, {
                props: {
                    initialCode,
                    isEmbedded,
                },
            });
        },
    });
};
