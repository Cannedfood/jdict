<script setup lang="ts">
import { computed } from '@vue/reactivity';

const props = defineProps<{
	kanji: string
}>();

const kanjiCharacters =
	computed(() =>
		props.kanji
		.split('')
		.filter(k => k.charCodeAt(0) >= 0x4e00 && k.charCodeAt(0) < 0x9FBF)
	);

</script>

<template lang="pug">
.kanji-info
	h5 {{kanji}}
	.detail-select(v-if="kanjiCharacters.length > 1")
		.detail-selectable(v-for="k of kanjiCharacters")
			| {{k}}
	.detail
</template>

<style lang="scss" scoped>

</style>
