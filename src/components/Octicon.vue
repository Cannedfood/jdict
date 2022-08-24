<script setup lang="ts">
import type { IconName } from '@primer/octicons'
import octicons from '@primer/octicons'
import { computed } from '@vue/reactivity';

const props = defineProps<{
	type: IconName
}>();

const svgCode = computed(() => {
	const code = octicons[props.type].toSVG();
	return code.replaceAll('<path ', '<path fill="currentColor"');
});

</script>

<template lang="pug">
.my-octicon(v-html="svgCode")
</template>
