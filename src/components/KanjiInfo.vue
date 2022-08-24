<script setup lang="ts">
import { computed } from '@vue/reactivity';
import { is_cjk } from '../util/Unicode';

const props = defineProps<{
	kanji: string
}>();

const kanjiCharacters =
	computed(() =>
		props.kanji
		.split('')
		.filter(k => is_cjk(k.codePointAt(0) ?? 0))
	);

</script>

<template lang="pug">
.kanji-info
	.detail-select(v-if="kanjiCharacters.length > 1")
		div(v-for="k of kanjiCharacters")
			| {{k}}
	.detail
</template>

<style lang="scss" scoped>
.detail-select {
	display: flex;
	flex-flow: row;
	>* {
		cursor: pointer;
		padding: .5em;
		margin: .1em;
		background: #8884;
	}
}
</style>
