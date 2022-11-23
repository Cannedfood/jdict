<script setup lang="ts">
import type { Gloss, Sense } from '../../backend/jmdict';
import { replaceEntities } from '../../backend/jmdict';

const props = defineProps<{
	senses: Sense[]
}>();

function generateGlossText(g: Gloss) {
	let result = g.value;

	const extra = [ g.type, g.gender ].filter(v => v);
	if(extra.length > 0)
		result += ` (${extra.join(', ')})`;

	return result;
}
</script>

<template lang="pug">
.meaning
	ol.senses
		li.sense(v-for="sense of senses")
			.content
				| {{sense.glosses.map(generateGlossText).join('; ')}}
				.extra-info &nbsp;
					.restrict-kanji(v-if="sense.restrict_to_kanji")
						| Only {{sense.restrict_to_kanji.join(', ')}}
					.restrict-reading(v-if="sense.restrict_to_reading")
						| Only {{sense.restrict_to_reading.join(', ')}}
					//- Stuff like "na-adjective"
					//- .part-of-speech(v-if="sense.part_of_speech_tags")
					//- 	| Part of Speech: {{sense.part_of_speech_tags.map(replaceEntities).join(', ')}}
					.fields(v-if="sense.fields")
						| {{sense.fields.map(replaceEntities).join(', ')}}
					.misc(v-if="sense.misc")
						| {{sense.misc.map(replaceEntities).join(', ')}}
					.sense-info(v-if="sense.info")
						| {{sense.info.map(replaceEntities).join(', ')}}
					.source-lang(v-if="sense.origin")
						| Source Language: {{sense.origin.map(o => `${o.lang}: ${o.word}`)}}
					.dialects(v-if="sense.dialects")
						| Dialect: {{sense.dialects?.map(replaceEntities).join(', ')}}
					.examples(v-if="sense.examples")
						.example(v-for="ex of sense.examples")
							| Example: {{ex.sentences}}
					.crossref(v-if="sense.xrefs")
						| See also&nbsp;
						a(v-for="xref in sense.xrefs" :href="`#/search/${xref.split('・')[0]}`") {{xref}}
					.antonyms(v-if="sense.antonyms")
						| Antonyms:
						a(v-for="ant in sense.antonyms" :href="`#/search/${ant.split('・')[0]}`") {{ant}}
</template>
