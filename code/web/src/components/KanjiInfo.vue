<script setup lang="ts">
import type { Character, ReadingMeaningGroup, ReadingType } from '@/backend/kanjidic';
import type { Kanji } from '@/backend/kanjivg';
import { ref } from 'vue';
import KanjiCompositionNode from './kanji_info/KanjiCompositionNode.vue';

const props = defineProps<{
	kanji: Character,
	kanjivg?: Kanji,
}>();

const hidden = [ 'korean_h', 'korean_r', 'vietnam' ];
function readings(g: ReadingMeaningGroup, type: ReadingType) {
	if(!g.readings) return undefined;
	if(hidden.includes(type)) return undefined;

	const result = g.readings.filter(r => r.typ == type).map(r => r.value);
	return result.length? result : undefined;
}

function zipKoreanReadings(g: ReadingMeaningGroup) {
	const hangul   = readings(g, 'korean_h');
	const reading  = readings(g, 'korean_r');
	if(!hangul || !reading) return undefined;
	return hangul.map((h, i) => `${h} (${reading[i]})`)
}

const expanded = ref(false);

</script>

<template lang="pug">
.kanji-info
	.kanji {{kanji.literal}}
	.tags
		.pill(v-if="kanji.misc.jlpt") JLPT {{kanji.misc.jlpt}}
		.pill(v-if="kanji.misc.grade") Grade {{kanji.misc.grade}}
		.pill(v-if="kanji.misc.freq") Freq: {{kanji.misc.freq}}
		.pill(v-if="kanji.misc.stroke_count && kanji.misc.stroke_count.length > 0")
			| Strokes: {{kanji.misc.stroke_count[0]}}
			//- span(v-if="kanji.stroke_count.length > 1")
			//- 	| ({{kanji.stroke_count.slice(1).join(', ')}})
	.info
		.mr_group(v-for="g of kanji.reading_meaning_groups")
			.meaning {{g.meanings?.filter(m => m.lang == 'en').map(m => m.value).join(', ')}}
			.reading-kun(v-if="readings(g, 'ja_kun')") On: {{readings(g, 'ja_kun')?.join(', ')}}
			.reading-on(v-if="readings(g, 'ja_on')") Kun: {{readings(g, 'ja_on')?.join(', ')}}
			.nanori(v-if="g.nanori?.length") Nanori: {{g.nanori?.join(', ')}}
			.reading-korean(v-if="zipKoreanReadings(g)") Korean: {{zipKoreanReadings(g)?.join(', ')}}
			.reading-vietnam(v-if="readings(g, 'vietnam')") Vietnamese: {{readings(g, 'vietnam')?.join(', ')}}
			.reading-pinyin(v-if="expanded && readings(g, 'pinyin')") Pinyin: {{readings(g, 'pinyin')?.join(', ')}}
		.radical(v-if="kanji.misc.rad_name?.length") Radical: {{kanji.misc.rad_name.join(', ')}}
		.decomposition(v-if="expanded && kanjivg")
			| Decomposition:
			KanjiCompositionNode(:kanjivg="kanjivg")
	.bonus-info(v-if="expanded")
		.codepoint(
			v-if="kanji.codepoint"
			v-for="value, key of kanji.codepoint"
		) {{key}}: {{value}}
	.expand-btn(role="button" @click="expanded = !expanded") 
		| {{expanded? 'Less' : 'More'}}
</template>

<style lang="scss" scoped>
.kanji-info {
	display: grid;
	grid-template-columns: min-content;
	grid-template-rows: min-content;
	grid-template-areas:
		"kanji info"
		"kanji info"
		"tags  info"
		"tags  bonus"
		"more  more";

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
		font-size: .8em;
	}
	.info {
		grid-area: info;
		.mr_group {
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

	.expand-button {
		grid-area: more;
	}
}
</style>
