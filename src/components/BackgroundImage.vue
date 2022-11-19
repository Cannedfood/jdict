<script setup lang="ts">
import { computed, ref, watch, watchEffect } from 'vue';
import { useMatchMedia } from '@/util/UseMedia';

const props = defineProps<{
	visible: boolean;
}>();

const loaded = ref(false);
const wasVisible = ref(false);
watchEffect(() => wasVisible.value ||= props.visible);

const dark_mode = useMatchMedia('(prefers-color-scheme: dark)');

const url = computed(() =>
	!wasVisible.value ? '' :
	dark_mode.value? '/japanese-street--unsplash-oCZHIa1D4EU.jpg' :
	/* light mode */ 'cherry-blossoms--unsplash-McsNra2VRQQ.jpg'
);

const log = (...args: any[]) => console.log(...args);

</script>

<template lang="pug">
img.website-background(
	:class="{ visible: visible && loaded }"
	@loadstart="loaded = false"
	@load="loaded = true; log('Loaded')"
	:src="url"
	loading="lazy"
)
</template>

<style lang="scss">
.website-background {
	display: block;
	position: fixed;
	top: 0;
	left: 0;
	height: 100vh;
	width: 100vw;
	z-index: -1;

	object-fit: cover;

	opacity: 0;
	&.visible { opacity: 1; }
	transition: opacity var(--home-transition-time) ease-out;
}
</style>

