<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Entry, Gloss, Kanji } from '../backend/jmdict'
import { replaceEntities } from '../backend/jmdict';
import KanjiText from './entry/KanjiText.vue';
import MeaningText from './entry/MeaningText.vue';

const props = defineProps<{ entry: Entry }>();

const senses = computed(() => props.entry.senses.filter(s => s.glosses.some(g => !g.lang)))

const highlightReading = computed(() =>
	props.entry.kanji.every(k => k.infos?.includes('&rK;')) ||
	senses.value.every(s => s.misc_info?.includes('&uk;')) // "word usually written using kana alone"
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
		.kanji
			span(v-if="highlightReading" v-for="kanji, i in entry.readings")
				span(:class="kanjiClasses(kanji)")
					span {{kanji.value}}
					span(v-if="i + 1 != entry.kanji.length") ,&nbsp;
			span(v-for="kanji, i in entry.kanji")
				KanjiText(:kanji="kanji" :class="kanjiClasses(kanji)")
					h1 Kapow!
				span(v-if="i + 1 != entry.kanji.length") ,&nbsp;
	MeaningText(:senses="senses")
	.debug-info.float-right(v-if="showDebugInfo")
		p rating: {{entry.rating?.toString(10)}}
		p(v-if="showJson") {{entry}}
</template>

<style lang="scss">
.entry {
	margin-bottom: 2em;

	.debug-info {
		display: none;
		opacity: 50%;
	}
	&:hover {
		.debug-info {
			display: block;
		}
	}
	.floating-container {
		position: absolute;
		background: #242424;
		width: 100%;
		max-width: 20cm;
		border: 2px solid white;
		z-index: 1;
	}
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

	.reading {
		width: fit-content;
		color: #FFF9;
		font-size: .7rem;
	}
	.kanji {
		font-size: 1.2rem;
		.phoneticReading { font-weight: bold; }
		.irregularUsage { opacity: 60%; }
		.rarelyUsed { opacity: 30%; }
	}
	.extra-info {
		display: inline;
		opacity: 30%;
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
