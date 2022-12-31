<script setup lang="ts">
import type { Kanji } from '@/backend/kanjivg';

const prop = defineProps<{
  kanjivg: Kanji
}>()

</script>

<template lang="pug">
.node(:class="{ hasChildren: kanjivg.parts.length > 0 }")
    router-link(role="button" :to="'/search/' + kanjivg.kanji") {{kanjivg.kanji}}
    ul
        li(v-for="part of kanjivg.parts")
            KanjiCompositionNode(:kanjivg="part")
</template>

<style lang="scss" scoped>
.node {
    // border: 1px solid white;
    // border-radius: .3em;
    // padding: .2em .5em;
    // margin-inline: .1em;
    .kanji {
        width: 1.5em;
        height: 1.5em;
        text-align: center;
        border-radius: .2em;

        background: rgba(136, 136, 136, 0.2);
    }
    .child-groups {
        display: flex;
        flex-flow: wrap column;
    }
}
</style>
