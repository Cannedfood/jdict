<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Entry } from '../backend/jmdict';

const props = defineProps<{ entry: Entry }>();

const reading = computed(() => props.entry.readings.map(r => r.value).join(', '));
const kanji = computed(() => (props.entry.kanji || props.entry.readings).map(r => r.value).join(', '));
const senses = computed(
	() => props.entry.senses.filter(s => s.glosses.some(g => !g.lang))
)

</script>

<template lang="pug">
.entry
	.word
		.reading(v-if="entry.kanji") {{reading}}
		.kanji {{kanji}}
	.meaning
		ol.senses
			li.sense(v-for="sense of senses")
				| {{sense.glosses.map(g => g.content).join('; ')}}
</template>

<style lang="scss">
.entry {
	margin-bottom: 3em;
}
.reading {
	color: #FFF9;
	font-size: .7rem;
}
.kanji {
	font-size: 1.2rem;
}
</style>
