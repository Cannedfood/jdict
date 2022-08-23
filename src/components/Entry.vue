<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Entry } from '../backend/jmdict';

const props = defineProps<{ entry: Entry }>();

const reading = computed(() => props.entry.readings.map(r => r.value).join(', '));
const readingRomaji = computed(() => props.entry.readings.map(r => r.romaji).join(', '));
const kanji = computed(() => (props.entry.kanji || props.entry.readings).map(r => r.value).join(', '));
const senses = computed(
	() => props.entry.senses.filter(s => s.glosses.some(g => !g.lang))
)

</script>

<template lang="pug">
.entry
	.word
		.reading(v-if="entry.kanji")
			.reading-entry(v-for="reading, i of entry.readings")
				.romaji {{reading.romaji}}
				.kana   {{reading.value + (i + 1 != entry.readings.length? ', ' : '')}}
		.kanji {{kanji}}
	.meaning
		ol.senses
			li.sense(v-for="sense of senses")
				| {{sense.glosses.map(g => g.content).join('; ')}}
</template>

<style lang="scss" scoped>
.entry {
	margin-bottom: 2em;
}
.reading {
	width: fit-content;
	color: #FFF9;
	font-size: .7rem;

	.reading-entry {
		display: inline-block;
		.romaji {
			opacity: 0;
			transition: opacity 100ms;
		}
		&:hover .romaji {
			opacity: 100%;
		}
	}
}
.kanji {
	font-size: 1.2rem;
}
</style>
