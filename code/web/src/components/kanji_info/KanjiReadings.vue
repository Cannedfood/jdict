<script setup lang="ts">
import type { Character, ReadingMeaningGroup, ReadingType } from '@/backend/kanjidic';

const props = defineProps<{
	kanji: Character,
    expanded: boolean,
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

</script>

<template lang="pug">
.mr_group(v-for="g of kanji.reading_meaning_groups")
	.meaning {{g.meanings?.filter(m => m.lang == 'en').map(m => m.value).join(', ')}}
	.reading-kun(v-if="readings(g, 'ja_kun')") On: {{readings(g, 'ja_kun')?.join(', ')}}
	.reading-on(v-if="readings(g, 'ja_on')") Kun: {{readings(g, 'ja_on')?.join(', ')}}
	.nanori(v-if="g.nanori?.length") Nanori: {{g.nanori?.join(', ')}}
	.reading-korean(v-if="zipKoreanReadings(g)") Korean: {{zipKoreanReadings(g)?.join(', ')}}
	.reading-vietnam(v-if="readings(g, 'vietnam')") Vietnamese: {{readings(g, 'vietnam')?.join(', ')}}
	.reading-pinyin(v-if="expanded && readings(g, 'pinyin')") Pinyin: {{readings(g, 'pinyin')?.join(', ')}}
</template>

<style lang="scss" scoped>
.mr_group {
	.meaning {
		margin-block: 1em;
		font-size: 1.3em;
		// font-weight: bold;
	}
}
</style>
