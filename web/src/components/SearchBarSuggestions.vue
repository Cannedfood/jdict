<script setup lang="ts">
import { computed, ref } from 'vue';

const props = defineProps<{
	suggestions: string[]
}>();

const emit = defineEmits<{
	(e: 'accept', v: string): void
}>();

const suggestionOffset = ref(0);
const suggestionCount = ref(5);
const selectedSuggestion = ref(0);
const shownSuggestions = computed(() => (
	props.suggestions
	.slice(suggestionOffset.value, suggestionOffset.value + suggestionCount.value)
	.map((s, i) => [s, i + suggestionOffset.value] as [string, number])
));
function fixSuggestionOffset() {
	selectedSuggestion.value = Math.min(Math.max(selectedSuggestion.value, 0), props.suggestions.length - 1);

	suggestionOffset.value =
	Math.max(0,
		Math.min(props.suggestions.length - suggestionCount.value,
			suggestionOffset.value
		)
	);

	if(props.suggestions.length < suggestionCount.value)
		suggestionOffset.value = 0;
	else if(selectedSuggestion.value < suggestionOffset.value)
		suggestionOffset.value = selectedSuggestion.value;
	else if(selectedSuggestion.value >= suggestionOffset.value + suggestionCount.value)
		suggestionOffset.value = selectedSuggestion.value - suggestionCount.value + 1;
}
function suggestionUp() {
	selectedSuggestion.value--;
	fixSuggestionOffset();
}
function suggestionDown() {
	selectedSuggestion.value++;
	fixSuggestionOffset();
}
function acceptSuggestion(suggestion?: string) {
	const query = suggestion ?? props.suggestions[selectedSuggestion.value];
	emit('accept', query);
}

defineExpose({
	suggestionUp, suggestionDown, acceptSuggestion
});

</script>

<template lang="pug">
.suggestions(v-if="suggestions && suggestions.length")
	//- | {{selectedSuggestion}}, {{suggestionOffset}} {{suggestionCount}}
	.suggestion(
		v-for="[suggestion, i] of shownSuggestions"
		:class="{ active: i == selectedSuggestion }"
		@click="acceptSuggestion(suggestion)"
		@mouseenter="selectedSuggestion = i"
	)
		| {{suggestion}}
		span.tab-hint [tab]
</template>

<style lang="scss" scoped>
.suggestion {
	cursor: pointer;
	.tab-hint {
		font-size: .5em;
		opacity: 0;
		transition: opacity 200ms;
		float: right;
	}
	&.active {
		text-decoration: underline;
		.tab-hint {
			opacity: 100%;
		}
	}
}
</style>
