<script setup lang="ts">
import type { Character, ReadingMeaningGroup, ReadingType } from '@/backend/kanjidic';
import type { Kanji } from '@/backend/kanjivg';
import { ref } from 'vue';
import KanjiDecomposition from './kanji_info/KanjiDecomposition.vue';
import KanjiReadings from './kanji_info/KanjiReadings.vue';
import StrokeOrder from './kanji_info/StrokeOrder.vue';

const props = defineProps<{
	kanji: Character,
	kanjivg?: Kanji,
	startExpanded?: boolean,
}>();

const expanded = ref(props.startExpanded ?? false);

</script>

<template lang="pug">
.kanji-info(:class="{expanded}" @click="!expanded && (expanded = true)")
	.kanji {{kanji.literal}}
	.tags
		.strokes(v-if="expanded && kanjivg")
			StrokeOrder(:kanjivg="kanjivg")
		.pill(v-if="kanji.misc.jlpt") JLPT {{kanji.misc.jlpt}}
		.pill(v-if="kanji.misc.grade") Grade {{kanji.misc.grade}}
		.pill(v-if="kanji.misc.freq") Freq: {{kanji.misc.freq}}
		.pill(v-if="kanji.misc.stroke_count && kanji.misc.stroke_count.length > 0")
			| Strokes: {{kanji.misc.stroke_count[0]}}
			//- span(v-if="kanji.stroke_count.length > 1")
			//- 	| ({{kanji.stroke_count.slice(1).join(', ')}})
	.info
		KanjiReadings(:kanji="kanji" :expanded="expanded")
		.radical(v-if="kanji.misc.rad_name?.length") Radical: {{kanji.misc.rad_name.join(', ')}}
		.decomposition(v-if="expanded && kanjivg")
			h5 Decomposition:
			KanjiDecomposition(:kanjivg="kanjivg")
	.expand-btn(role="button" @click.stop="expanded = !expanded")
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

	cursor: pointer;
	&.expanded { cursor: unset; }

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
	}

	.expand-button {
		grid-area: more;
	}
}
</style>
