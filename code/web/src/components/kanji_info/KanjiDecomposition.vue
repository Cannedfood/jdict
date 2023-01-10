<script setup lang="ts">
import type { Kanji } from '@/backend/kanjivg';
import KanjiDecompositionNode from './KanjiDecompositionNode.vue';

const props = defineProps<{
  kanjivg: Kanji
}>()

</script>

<template lang="pug">
.decomposition.text-center
  | Decomposition:
  KanjiDecompositionNode(:kanjivg="kanjivg")
</template>

<style lang="scss" scoped>
.decomposition {
  border: 1px solid #888;
  border-radius: .2em;
  width: fit-content;
  padding: .4em 1em;
  margin-block: 1em;
}
</style>
