<script setup lang="ts">
import type { Kanji } from '@/backend/kanjivg';

const props = defineProps<{
  kanjivg: Kanji
}>()

</script>

<template lang="pug">
.node
    RouterLink.kanji(:to="'/search/' + kanjivg.kanji")
        | {{ kanjivg.kanji }}
    .child-groups(v-if="kanjivg.parts && kanjivg.parts.length > 0")
        KanjiDecompositionNode(
            v-for="child in kanjivg.parts"
            :kanjivg="child"
        )
</template>

<style lang="scss" scoped>
.node {
    width: fit-content;
    margin: 0 .3em;
    &:first-child { margin-left: 0; }
    &:last-child { margin-right: 0; }

    .kanji {
        padding: .3em;
        font-size: 1.5em;
        border-radius: .2em;
        margin-inline: auto;

        text-align: center;
        text-decoration: none;

        background: rgba(136, 136, 136, 0.2);

        &:hover {
            background: rgba(136, 136, 136, 0.4);
        }
    }
    .child-groups {
        display: flex;
        flex-flow: wrap row;
        border-top: 1px solid #888;
        border-radius: .1em;
    }
}
</style>
