<script setup lang="ts">
import { computed } from '@vue/reactivity';
import { ref } from 'vue';
import type { Kanji } from '@/backend/jmdict';
import { is_cjk } from '@/util/Unicode';

const props = defineProps<{
	kanji: Kanji
}>();

</script>

<template lang="pug">
span.kanji
	a(v-for="c, i in kanji.value.split('')"
		:class="{ hanCharacter: is_cjk(c.codePointAt(0) ?? 0) }"
	) {{c}}
</template>

<style lang="scss">
.kanji {
	.hanCharacter {
		cursor: help;
		&:hover {
			font-weight: bolder;
		}
	}
}
</style>
