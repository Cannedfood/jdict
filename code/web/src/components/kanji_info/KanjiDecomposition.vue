<script setup lang="ts">
import type { Kanji } from '@/backend/kanjivg';
import KanjiDecompositionNode from './KanjiDecompositionNode.vue';

const props = defineProps<{
  kanjivg: Kanji
}>()

</script>

<template lang="pug">
.decomposition
    KanjiDecompositionNode(:kanjivg="kanjivg")
</template>

<style lang="scss" scoped>
.decomposition {
    border: 1px solid white;
    border-radius: .2em;
    width: fit-content;
    padding: 1em;
}
</style>
