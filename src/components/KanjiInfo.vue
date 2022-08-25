<script setup lang="ts">
import type { Character, VariantType } from '@/backend/kanjidic';

const props = defineProps<{
	kanji: Character
}>();

// function variants(t: VariantType) {
// 	return props.kanji.variant?.filter(v => v.type == t).map(v => v.value) ?? [];
// }

</script>

<template lang="pug">
.kanji-info
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
			.reading-kun On: {{g.readings?.filter(r => r.type == 'ja_kun').map(r => r.value).join(', ')}}
			.reading-on Kun: {{g.readings?.filter(r => r.type == 'ja_on').map(r => r.value).join(', ')}}
			.meaning {{g.meanings?.filter(m => !m.lang).map(m => m.value).join(', ')}}
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
	grid-template-columns: min-content 5fr;
	grid-template-rows: min-content 3em;
	grid-template-areas:
		"kanji info"
		"kanji info"
		"tags  info"
		"bonus bonus";

	border: 1px solid #888;
	border-radius: .5em;
	margin-top: 4em;
	padding: .5em;
	&>* { margin: .5rem; }
	.kanji {
		grid-area: kanji;

		text-align: center;
		font-size: 3.5em;
		width: 5rem;
		height: 5rem;

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
