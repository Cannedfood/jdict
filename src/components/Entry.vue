<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Entry, Kanji } from '../backend/jmdict'
import { replaceEntities } from '../backend/jmdict';
import OnHover from './OnHover.vue';
import KanjiInfo from './KanjiInfo.vue';

const props = defineProps<{ entry: Entry }>();

const senses = computed(() => props.entry.senses.filter(s => s.glosses.some(g => !g.lang)))

const hasGoodKanji = computed(() => props.entry.kanji.some(k => !k.infos?.includes('&rK;')));

function kanjiClasses(kanji: Kanji) {
	return {
		rarelyUsed: kanji.infos?.some(e => ['&rK;', '&sK;', '&oK;'].includes(e)),
		irregularUsage: kanji.infos?.some(e => [ '&ik;', '&iK;', '&io;' ].includes(e)),
		phoneticReading: kanji.infos?.some(e => [ '&ateji;' ].includes(e))
	};
}

const showDebugInfo = window.location.hostname.startsWith("localhost");

</script>

<template lang="pug">
.entry
	.word
		.reading
			.reading-entry(v-for="reading, i of entry.readings")
				.romaji {{reading.romaji}}
				.kana   {{reading.value + (i + 1 != entry.readings.length? ', ' : '')}}
		.kanji
			span(v-if="!hasGoodKanji" v-for="kanji, i in entry.readings")
				span(:class="kanjiClasses(kanji)")
					span {{kanji.value}}
					span(v-if="i + 1 != entry.kanji.length") ,&nbsp;
			span(v-for="kanji, i in entry.kanji")
				OnHover
					span(:class="kanjiClasses(kanji)")
						span {{kanji.value}}
						span(v-if="i + 1 != entry.kanji.length") ,&nbsp;
					template(v-slot:when-hovered)
						.floating-container
							p prio: {{kanji.priorities?.map(replaceEntities).join(', ')}}
							p info: {{kanji.infos?.map(replaceEntities).join(', ')}}
							KanjiInfo(:kanji="kanji.value")
	.meaning
		ol.senses
			li.sense(v-for="sense of senses")
				.content
					| {{sense.glosses.map(g => g.content).join('; ')}}
					| {{sense.dialects?.map(replaceEntities).map(k => '['+k+']')}}
				.crossref(v-if="sense.cross_references")
					| See also&nbsp;
					a(v-for="xref in sense.cross_references" :href="`#/search/${xref.split('・')[0]}`") {{xref}}
	.debug-info(v-if="showDebugInfo")
		p {{entry.rating}}
</template>

<style lang="scss" scoped>
.debug-info {
	display: none;
	opacity: 50%;
}
.entry:hover {
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
}

.entry {
	margin-bottom: 2em;
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
</style>
