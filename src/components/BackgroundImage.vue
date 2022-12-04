<script setup lang="ts">
import { computed } from 'vue';
import { useMatchMedia } from '@/util/UseMedia';
import ImageLodChain from './ImageLodChain.vue';
import { BackgroundImages } from './BackgroundImages'
import { sample } from 'lodash';
import { useViewportSize } from '@/util/ViewportSize';

const props = defineProps<{
	visible: boolean;
}>();

const isDarkTheme = useMatchMedia('(prefers-color-scheme: dark)');
const preferReducedData = useMatchMedia('(prefers-reduced-data: reduce)');
const viewportSize = useViewportSize();

const backgroundDark   = sample(Object.values(BackgroundImages).filter(x => x.style == 'dark'));
const backgroundBright = sample(Object.values(BackgroundImages).filter(x => x.style == 'light'));

const background = computed(() =>
	preferReducedData.value? undefined :
	isDarkTheme.value? backgroundDark :
	backgroundBright
);
const lodChain = computed(() => {
	const levels = background.value?.levels;
	if(!levels || levels.length == 0)
		return [];

	const initial = levels[0];
	const optimal = (() => {
		for(const level of levels) {
			if(level.width >= viewportSize.width && level.height >= viewportSize.height)
				return level;
		}
		return levels[levels.length - 1];
	})();

	if(initial.url == optimal.url)
		return [initial.url];
	else
		return [initial.url, optimal.url];
});

</script>

<template lang="pug">
.website-backgrounds(v-if="visible" :class="{ visible }")
	ImageLodChain(
		alt="Background image"
		:urls="lodChain"
	)
	.credit(v-if="background?.credit" v-html="background?.credit")
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

	.credit {
		position: absolute;
		bottom: 0;
		left: 0;

		background: var(--layer1);
		color: var(--text1);

		padding: 0.5em;
		border-top-right-radius: .5em;
	}
}
</style>

