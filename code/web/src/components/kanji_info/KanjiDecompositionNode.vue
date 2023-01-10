<script setup lang="ts">
import type { Kanji } from '@/backend/kanjivg';

const props = defineProps<{
  kanjivg: Kanji
}>()

</script>

<template lang="pug">
.node
    .kanji {{ kanjivg.kanji }}
    .child-groups(v-if="kanjivg.parts && kanjivg.parts.length > 0")
        KanjiDecompositionNode(
            v-for="child in kanjivg.parts"
            :kanjivg="child"
        )
</template>

<style lang="scss" scoped>
.node {
    width: fit-content;

    .kanji {
        width: 1.5em;
        height: 1.5em;
        text-align: center;
        border-radius: .2em;

        margin-inline: auto;

        background: rgba(136, 136, 136, 0.2);
    }
    .child-groups {
        display: flex;
        flex-flow: wrap row;
        border-top: 1px solid white;
        // border-inline: 1px solid gray;
        margin-inline: .4em;
    }
}
</style>
