<script setup lang="ts">
import type { Kanji } from '@/backend/jmdict';
import { is_cjk } from '@/util/Unicode';
import { computed } from '@vue/reactivity';

const props = defineProps<{
	kanji: Kanji
}>();

const sections = computed(() => {
	const result = [];

	let boringText = '';
	function flush() {
		if(boringText.length)
			result.push(boringText);
	}

	for(let c of props.kanji.value.split('')) {
		if(isKanji(c)) {
			flush();
			result.push(c);
		}
		else
			boringText += c;
	}

	return result;
})

function isKanji(s: string) {
	return is_cjk(s.codePointAt(0) ?? 0);
}

</script>

<template lang="pug">
span.kanji-text
	template(v-for="s in sections")
		router-link(
			v-if="isKanji(s)"
			class="kanji-character"
			:to="`/#/search/${s}`"
		) {{s}}
		span(v-else) {{s}}
</template>

<style lang="scss">
.kanji-text {
	.kanji-character {
		text-decoration: none;
		cursor: pointer;
		&:hover {
			font-weight: bolder;
		}
	}
}
</style>
