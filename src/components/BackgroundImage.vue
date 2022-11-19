<script setup lang="ts">
import { computed, ref, watch, watchEffect } from 'vue';
import { useMatchMedia } from '@/util/UseMedia';
import ImageLodChain from './ImageLodChain.vue';

const props = defineProps<{
	visible: boolean;
}>();

const isDarkTheme = useMatchMedia('(prefers-color-scheme: dark)');

const backgroundDark = [
	// '/japanese-street--unsplash-oCZHIa1D4EU-small.webp',
	'/japanese-street--unsplash-oCZHIa1D4EU-medium.webp',
	'/japanese-street--unsplash-oCZHIa1D4EU-high.webp',
];
const backgroundBright = [
	// '/cherry-blossoms--unsplash-McsNra2VRQQ-small.webp',
	'/cherry-blossoms--unsplash-McsNra2VRQQ-medium.webp',
	'/cherry-blossoms--unsplash-McsNra2VRQQ-high.webp',
];

const lodChain = computed(() => isDarkTheme.value? backgroundDark : backgroundBright);

const highest_loaded = ref(-1);

const log = console.log;

</script>

<template lang="pug">
.website-backgrounds(:class="{ visible }")
	ImageLodChain(:urls="lodChain")
</template>

<style lang="scss" scoped>
.website-backgrounds {
	position: fixed;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;
	z-index: -1;

	opacity: 0;
	&.visible {
		opacity: 1;
		transition: opacity var(--home-transition-time) ease-out;
	}
}
</style>

