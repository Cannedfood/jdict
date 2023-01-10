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
	.left
		.kanji {{kanji.literal}}
		.tags
			.strokes(v-if="expanded && kanjivg")
				StrokeOrder(:kanjivg="kanjivg")
			.pill(v-if="kanji.misc.jlpt") JLPT {{kanji.misc.jlpt}}
			.pill(v-if="kanji.misc.grade") Grade {{kanji.misc.grade}}
			.pill(v-if="kanji.misc.freq") Freq: {{kanji.misc.freq}}
			.pill(v-if="kanji.misc.stroke_count && kanji.misc.stroke_count.length > 0")
				| Strokes: {{kanji.misc.stroke_count[0]}}
				span(v-if="kanji.misc.stroke_count.length > 1")
					| ({{kanji.misc.stroke_count.slice(1).join(', ')}})
	.info
		KanjiReadings(:kanji="kanji" :expanded="expanded")
		.radical(v-if="kanji.misc.rad_name?.length") Radical: {{kanji.misc.rad_name.join(', ')}}
		.decomposition(v-if="expanded && kanjivg")
			h5 Decomposition:
			KanjiDecomposition(:kanjivg="kanjivg")
	.more(@click.stop="expanded = !expanded")
		.chevron(:class="{ up: expanded }") ❯
</template>

<style lang="scss" scoped>
.kanji-info {
	// Styling
	border: 1px solid #888;
	border-radius: .5em;
	margin-block: .5em;
	padding: .5em;
	&>* { margin: .5rem; }

	&:not(.expanded) {
		cursor: pointer;
		// box-shadow: inset 0 -2em 2em -2em #000;
	}

	// Layout
	display: grid;
	grid-template-columns: min-content auto;
	grid-template-rows: 1fr 2em;
	grid-template-areas:
		"left info"
		"more more";
	.left { grid-area: left; }
	.info { grid-area: info; }
	.more { grid-area: more; }

	// Children
	.kanji {
		text-align: center;
		font-size: 3.5em;
		width: 6rem;
		height: 6rem;

		background: #8883;
		border-radius: .2rem;
	}
	.tags {
		font-size: .8em;
		margin-top: 1em;
	}
	.more {
		text-align: center;
		cursor: pointer;
	}
}
</style>
