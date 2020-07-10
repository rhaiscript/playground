<style scoped>
.tabsContainer {
    display: grid;
    grid-template-columns: 100%;
    grid-template-rows: auto minmax(0, 1fr);
}
.tabsContainer > .tabs {
    margin-bottom: 0;
    grid-column: 1 / span 2;
    grid-row: 1;
}
.tabsContainer > .tabs li.is-disabled {
    pointer-events: none;
    cursor: not-allowed;
    opacity: 0.5;
}
.tabsContainer > .tabItem {
    grid-column: 1;
    grid-row: 2;
}
/* Horizontal split */
.tabsContainer.splitted {
    grid-template-columns: 50% 50%;
    grid-template-rows: auto minmax(0, 1fr);
}
.tabsContainer.splitted > .tabs {
    grid-column: 2;
    grid-row: 1;
}
.tabsContainer.splitted > .tabItem {
    grid-column: 2;
    grid-row: 2;
}
.tabsContainer.splitted > .tabItem.splittable {
    grid-column: 1;
    grid-row: 1 / span 2;
}
/* Vertical split */
.tabsContainer.splitted.vertical {
    grid-template-columns: 100%;
    grid-template-rows: minmax(0, 1fr) auto minmax(0, 1fr);
}
.tabsContainer.splitted.vertical > .tabs {
    grid-column: 1;
    grid-row: 2;
}
.tabsContainer.splitted.vertical > .tabItem {
    grid-column: 1;
    grid-row: 3;
}
.tabsContainer.splitted.vertical > .tabItem.splittable {
    grid-column: 1;
    grid-row: 1;
}
</style>

<template>
    <div class="tabsContainer" :class="{ splitted: isSplitted, vertical: isVerticalSplit }">
        <nav class="tabs">
            <ul>
                <li
                    v-for="(tab, i) in tabItems"
                    :key="i"
                    v-show="!tab.splittable || !isSplitted"
                    :class="{ 'is-active': tab.isActive, 'is-disabled': tab.disabled }"
                >
                    <a href="#" @click.prevent="makeTabActive(i)">
                        <span>{{ tab.label }}</span>
                    </a>
                </li>
            </ul>
        </nav>
        <slot></slot>
    </div>
</template>

<script>
import TabItem from "./TabItem.vue";

export default {
    props: {
        layout: {
            type: String,
            required: true,
            validator(val) {
                return ['auto', 'h', 'v', 'tabbed'].includes(val);
            },
        },
    },
    data() {
        return {
            _autoIsSplitted: false,
            tabItems: [],
            activeTab: -1,
        };
    },
    computed: {
        isSplitted() {
            if (this.layout === "auto") {
                return this.$data._autoIsSplitted;
            } else {
                return this.layout === "h" || this.layout === "v";
            }
        },
        isVerticalSplit() {
            return this.layout === "v";
        },
    },
    watch: {
        tabItems(newVal) {
            this.tabItems.forEach((tab, i) => {
                tab.isSplitted = tab.splittable && this.isSplitted;
                tab.isActive = false;
            });
            this.activeTab = this.tabItems.findIndex(tab => !tab.isSplitted);
            this._unsplittedActiveTab = 0;
        },
        activeTab(newVal) {
            this.tabItems.forEach((tab, i) => {
                tab.isActive = this.activeTab === i;
            });
            this.$emit("activeTabChanged", newVal);
        },
        isSplitted(newVal) {
            for (const tab of this.tabItems) {
                if (tab.splittable) {
                    tab.isSplitted = newVal;
                }
            }
            if (newVal) {
                if (this.activeTab >= 0 && this.tabItems[this.activeTab].isSplitted) {
                    // Store last active tab to revert to when unsplitting:
                    this._unsplittedActiveTab = this.activeTab;
                    this.activeTab = this.tabItems.findIndex(tab => !tab.isSplitted);
                }
            } else if (this._unsplittedActiveTab >= 0) {
                this.activeTab = this._unsplittedActiveTab;
            }
        },
    },
    methods: {
        refreshSlots() {
            if (this.$slots.default && this.$slots.default.length > 0) {
                this.tabItems = this.$slots.default
                    .filter(vnode => vnode.componentInstance)
                    .map(vnode => vnode.componentInstance);
            } else {
                this.tabItems = [];
            }
        },
        makeTabActive(idx) {
            if (typeof idx !== "number") {
                idx = this.tabItems.findIndex(tab => tab === idx);
                if (idx < 0) {
                    return;
                }
            }
            // When a tab is explicitly made active, we no longer need to revert
            // to the last active tab:
            this._unsplittedActiveTab = idx;
            if (!this.tabItems[idx].isSplitted) {
                this.activeTab = idx;
            }
        },
    },
    created() {
        const matchMedia = window.matchMedia("(min-width: 900px)");
        matchMedia.addListener(this._matchMediaListener = () => {
            this.$data._autoIsSplitted = matchMedia.matches;
        });
        this._matchMedia = matchMedia;
        this.$watch(function () {
            return this.isSplitted + this.isVerticalSplit;
        }, function () {
            this.$emit("layoutChanged");
        });
    },
    mounted() {
        this.$data._autoIsSplitted = this._matchMedia.matches
        this.refreshSlots();
    },
    destroyed() {
        this._matchMedia.removeListener(this._matchMediaListener);
    },
};
</script>
