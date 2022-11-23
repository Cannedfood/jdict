<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Entry, Kanji } from '../backend/jmdict'
import KanjiText from './entry/KanjiText.vue';
import MeaningText from './entry/MeaningText.vue';

const props = defineProps<{ entry: Entry }>();

// const senses = computed(() => props.entry.senses)
const senses = computed(() => props.entry.senses.filter(s => s.glosses.some(g => g.lang == 'eng')))

const highlightReading = computed(() =>
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

const showDebugInfo = window.location.search.includes("debug=");
const showJson = window.location.search.includes("debug=json")

</script>

<template lang="pug">
.entry
	.word
		.reading
			.reading-entry(v-for="reading, i of entry.readings")
				.romaji {{reading.romaji}}
				.kana   {{reading.value + (i + 1 != entry.readings.length? ', ' : '')}}
		.kanji(v-if="entry.kanji")
			span(v-if="highlightReading" v-for="reading, i in entry.readings")
				span(:class="kanjiClasses(reading)")
					span {{reading.value}}
					span(v-if="i + 1 != entry.kanji.length || entry.kanji.length > 0") ,&nbsp;
			span(v-for="kanji, i in entry.kanji")
				KanjiText(:kanji="kanji" :class="kanjiClasses(kanji)")
					h1 Kapow!
				span(v-if="i + 1 != entry.kanji?.length") ,&nbsp;
	MeaningText(:senses="senses")
</template>

<style lang="scss">
.entry {
	.word {
		// Used to allow floating the romaji text. See .romaji
		position: relative;
	}
	.debug-info {
		display: none;
		color: var(--text-muted1);
	}
	&:hover {
		.debug-info {
			display: block;
		}
	}
	.floating-container {
		position: absolute;
		background: var(--background1);
		width: 100%;
		max-width: 20cm;
		border: 2px solid white;
		z-index: 1;
	}
	.reading-entry {
		display: inline-block;
		.romaji {
			bottom: 100%;
			position: absolute;

			opacity: 0;
			transition: opacity 100ms;
		}
		&:hover .romaji {
			opacity: 1;
		}
	}

	.reading {
		width: fit-content;
		font-size: .7rem;
		color: var(--text-muted);
		top: 0;
	}
	.kanji {
		font-size: 1.2rem;
		.phoneticReading { font-weight: bold; }
		.irregularUsage { color: var(--text-muted1); }
		.rarelyUsed { color: var(--text-muted2); }
	}
	.extra-info {
		display: inline;
		color: var(--text-muted2);
		&> * {
			display: inline-block;
			margin-inline: .5em;
		}
		.fields, .crossref {
			display: block;
		}
	}

}
</style>
