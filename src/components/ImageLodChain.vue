<script setup lang="ts">
import { reactive } from 'vue';

const props = defineProps<{ urls: string[]; alt: string; }>();

const loadedLevels = reactive([] as number[]);
</script>

<template lang="pug">
.image-lod-chain
	img.lod-level(
		v-for="(url, i) in urls" :key="url" :src="url"
		@load="loadedLevels.push(i)"
		:alt="alt"
		:class=`{
			first: i == 0,
			loaded: loadedLevels.includes(i),
			superseeded: loadedLevels.some(j => j > i)
		}`
	)
</template>

<style lang="scss" scoped>
.image-lod-chain {
	position: relative;
}
.lod-level {
	position: fixed;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;

	width: 100%;
	height: 100%;
	object-fit: cover;

	opacity: 0;
	&.loaded {
		opacity: 1;
		transition: opacity .5s ease-out;
	}
	&.loaded.first {
		transition: opacity .2s ease-out;
		transition-delay: 0;
	}
	&.loaded.first.superseeded {
		opacity: 1 !important;
	}
}
</style>
