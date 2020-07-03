<template>
    <div v-show="isActive || isSplitted" :class="{ tabItem: true, splittable: splittable }">
        <slot></slot>
    </div>
</template>

<script>
export default {
    data() {
        return {
            isActive: false,
            isSplitted: false,
        };
    },
    props: {
        splittable: Boolean,
        label: String,
        disabled: Boolean,
    },
    methods: {
        makeTabActive() {
            this.$parent.makeTabActive(this);
        },
    },
    mounted() {
        this.$parent.refreshSlots()
    },
    destroyed() {
        this.$parent.refreshSlots();
    },
};
</script>
