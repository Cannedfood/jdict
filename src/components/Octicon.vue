<script setup lang="ts">
import { computed } from '@vue/reactivity';

import home from '@primer/octicons/build/svg/home-24.svg?raw'

const icons = {
	home
};

const props = defineProps<{
	type: keyof(typeof icons)
}>();

const svgCode = computed(() => {
	return icons[props.type].replaceAll('<path ', '<path fill="currentColor"');
});

</script>

<template lang="pug">
.my-octicon(v-html="svgCode")
</template>
