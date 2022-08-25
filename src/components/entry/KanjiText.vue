<script setup lang="ts">
import type { Kanji } from '@/backend/jmdict';
import { is_cjk } from '@/util/Unicode';

const props = defineProps<{
	kanji: Kanji
}>();

function isKanji(s: string) {
	return is_cjk(s.codePointAt(0) ?? 0);
}

</script>

<template lang="pug">
span.kanji
	a(
		v-for="c in kanji.value.split('')"
		:class="{ hanCharacter: isKanji(c) }"
		:href="isKanji(c)?`/#/search/${c}` : undefined"
	) {{c}}
</template>

<style lang="scss">
.kanji {
	.hanCharacter {
		text-decoration: none;
		cursor: help;
		&:hover {
			font-weight: bolder;
		}
	}
}
</style>
