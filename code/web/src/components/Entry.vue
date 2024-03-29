<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Entry, Kanji } from '../backend/jmdict'
import KanjiText from './entry/KanjiText.vue';
import MeaningText from './entry/MeaningText.vue';

const props = defineProps<{ entry: Entry }>();

// const senses = computed(() => props.entry.senses)
const senses = computed(() => props.entry.senses.filter(s => s.glosses.some(g => g.lang == 'eng')))

const highlightReading = computed(() =>
	!props.entry.kanji?.length ||
	props.entry.kanji?.every(k => k.infos?.includes('&rK;')) ||
	senses.value.every(s => s.misc?.includes('&uk;')) // "word usually written using kana alone"
);

function kanjiClasses(kanji: Kanji) {
	return {
		rarelyUsed: kanji.infos?.some(e => ['&rK;', '&sK;', '&oK;'].includes(e)),
		irregularUsage: kanji.infos?.some(e => [ '&ik;', '&iK;', '&io;' ].includes(e)),
		phoneticReading: kanji.infos?.some(e => [ '&ateji;' ].includes(e))
	};
}

</script>

<template lang="pug">
.entry
	.reading(v-if="!highlightReading")
		.reading-entry(v-for="reading, i of entry.readings")
			.romaji {{reading.romaji}}
			.kana   {{reading.value + (i + 1 != entry.readings.length? ', ' : '')}}
	.kanji
		span(v-if="highlightReading" v-for="reading, i in entry.readings")
			span(:class="kanjiClasses(reading)")
				span {{reading.value}}
				span(v-if="(entry.kanji?.length ?? 0) > 0") ,&nbsp;
		span(v-for="kanji, i in entry.kanji")
			KanjiText(:kanji="kanji" :class="kanjiClasses(kanji)")
			span(v-if="i + 1 != entry.kanji?.length") ,&nbsp;
	MeaningText(:senses="senses")
</template>

<style lang="scss" scoped>
.entry {
	position: relative;

	.reading {
		width: fit-content;
		font-size: .7rem;
		color: var(--text-muted);
		top: 0;
	}

	.reading-entry {
		display: inline-block;
		.romaji {
			bottom: 100%;
			position: absolute;

			opacity: 0;
			transition: opacity 100ms;
		}
		&:hover >.romaji {
			opacity: 1;
		}
	}

	.kanji {
		font-size: 1.2rem;
		.phoneticReading { font-weight: bold; }
		.irregularUsage { color: var(--text-muted1); }
		.rarelyUsed { color: var(--text-muted2); }
	}
}
</style>
