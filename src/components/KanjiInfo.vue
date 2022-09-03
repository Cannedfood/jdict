<script setup lang="ts">
import type { Character, ReadingMeaningGroup, ReadingType } from '@/backend/kanjidic';
import { ref } from 'vue';

const props = defineProps<{
	kanji: Character
}>();

const expanded = ref(false);


const hidden = [ 'korean_h', 'korean_r', 'vietnam' ];
function readings(g: ReadingMeaningGroup, type: ReadingType) {
	if(!g.readings) return undefined;
	if(hidden.includes(type)) return undefined;

	const result = g.readings.filter(r => r.type == type).map(r => r.value);
	return result.length? result : undefined;
}

function zipKoreanReadings(g: ReadingMeaningGroup) {
	const hangul   = readings(g, 'korean_h');
	const reading  = readings(g, 'korean_r');
	if(!hangul || !reading) return undefined;
	return hangul.map((h, i) => `${h} (${reading[i]})`)
}

</script>

<template lang="pug">
.kanji-info(:class="{expanded}")
	.kanji {{kanji.literal}}
	.tags
		span(v-if="kanji.jlpt") JLPT {{kanji.jlpt}}
		span(v-if="kanji.grade") Grade {{kanji.grade}}
		span(v-if="kanji.freq") Freq: {{kanji.freq}}
		span(v-if="kanji.stroke_count && kanji.stroke_count.length > 0")
			| Strokes: {{kanji.stroke_count[0]}}
			//- span(v-if="kanji.stroke_count.length > 1")
			//- 	| ({{kanji.stroke_count.slice(1).join(', ')}})
	.info
		.mr_group(v-for="g of kanji.reading_meaning_groups")
			.meaning {{g.meanings?.filter(m => !m.lang).map(m => m.value).join(', ')}}
			.reading-kun(v-if="readings(g, 'ja_kun')") On: {{readings(g, 'ja_kun')?.join(', ')}}
			.reading-on(v-if="readings(g, 'ja_on')") Kun: {{readings(g, 'ja_on')?.join(', ')}}
			.reading-korean(v-if="zipKoreanReadings(g)") Korean: {{zipKoreanReadings(g)?.join(', ')}}
			.reading-vietnam(v-if="readings(g, 'vietnam')") Vietnamese: {{readings(g, 'vietnam')?.join(', ')}}
			.reading-pinyin(v-if="readings(g, 'pinyin')") Pinyin: {{readings(g, 'pinyin')?.join(', ')}}
		.nanori(v-if="kanji.nanori") Nanori: {{kanji.nanori?.join(', ')}}
		.radical(v-if="kanji.rad_name") Radical: {{kanji.rad_name.join(', ')}}
	.bonus-info
		.codepoint(
			v-if="kanji.codepoint"
			v-for="value, key of kanji.codepoint"
		) {{key}}: {{value}}
</template>

<style lang="scss" scoped>
.kanji-info {
	display: grid;
	grid-template-columns: min-content;
	grid-template-rows: min-content;
	// grid-template-areas:
	// 	"kanji info"
	// 	"kanji info"
	// 	"tags  info"
	// 	"bonus bonus";
	grid-template-areas:
		"kanji info"
		"kanji info"
		"tags  info"
		"tags  bonus"
		"more  more";
	&.expanded {
		grid-template-areas:
			"kanji info"
			"kanji info"
			"tags  info"
			"tags  bonus"
			"more  more";
	}

	border: 1px solid #888;
	border-radius: .5em;
	margin-block: .5em;
	padding: .5em;
	&>* { margin: .5rem; }
	.kanji {
		grid-area: kanji;

		text-align: center;
		font-size: 3.5em;
		width: 6rem;
		height: 6rem;

		background: #8883;
		border-radius: .2rem;
	}
	.tags {
		grid-area: tags;
		&>span {
			white-space: nowrap;
			display: inline-block;
			background: #888;
			border-radius: .5em;
			padding-inline: .4em;
			margin-inline: .1em;
		}
	}
	.info {
		grid-area: info;
		.mr_group {
			margin-bottom: 2em;
			.meaning {
				margin-block: 1em;
				font-size: 1.3em;
				// font-weight: bold;
			}
		}
	}
	.bonus-info {
		grid-area: bonus;
		display: flex;
		flex-flow: wrap row;
		&>* {
			border: 1px solid #888;
			padding-inline: .2em;
			margin-inline: .2em;
		}
	}
}
</style>
