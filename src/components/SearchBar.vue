<script setup lang="ts">import { nextTick, onMounted, ref } from 'vue';

const props = defineProps<{
	modelValue: string,
	suggestions: string[]
}>();
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void,
  (e: 'send', value: string): void
}>()

function targetValue(e: any) { return e.target.value; }

const selectedSuggestion = ref(1);
function suggestionUp() { selectedSuggestion.value = Math.max(selectedSuggestion.value - 1, 1); }
function suggestionDown() { selectedSuggestion.value = Math.min(selectedSuggestion.value + 1, props.suggestions.length); }
function acceptSuggestion() {
	const query = props.suggestions[selectedSuggestion.value - 1];
	emit('update:modelValue', query);
	// emit('send', query);
}

const searchInput = ref<HTMLInputElement | null>(null);
onMounted(() => nextTick(() => searchInput.value?.focus()))
</script>

<template lang="pug">
.search-bar
	input(
		ref="searchInput"
		type="search"
		placeholder="Search..."
		:value="modelValue"
		autocomplete="off"
		@input="emit('update:modelValue', targetValue($event))"
		@keydown.enter="emit('send', modelValue)"
		@keydown.up="suggestionUp"
		@keydown.down="suggestionDown"
		@keydown.tab.prevent="acceptSuggestion"
		onfocus="this.select();"
	)
	button W
	a 漢
	select
		option EN
		option DE
		option RU
	.suggestions(v-if="suggestions")
		.suggestion(v-for="suggestion, i of suggestions" :class="{ active: i + 1 == selectedSuggestion }")
			span.tab-hint [tab]
			| {{suggestion}}
</template>

<style lang="scss">
.suggestions {
	position: absolute;
	top: 100%;

	.suggestion {
		.tab-hint {
			font-size: .5em;
			opacity: 0;
			transition: opacity 200ms;
		}
		&.active {
			text-decoration: underline;
			.tab-hint {
				opacity: 100%;
			}
		}
	}
}
.search-bar:not(:focus-within) {
	.suggestions {
		display: none;
	}
}

.search-bar {
	display: flex;
	flex-flow: nowrap row;

	position: relative;

	box-sizing: border-box;
	width: 100%;
	max-width: 14cm;

	margin: 0;
	margin-inline: auto;

	font-size: 2em;

	background: #0002;
	border-radius: .5em;
	padding-inline: .3em;
	padding-block: .2em;

	transition: 400ms border-color;
	border: 1px solid transparent;
	&:focus-within {
		border-color: purple;
	}

	* {
		font-size: inherit;
		margin: 0;
		padding-block: .1em;
		padding-inline: .1em;
		border-radius: 0;
	}
	input, button, select, a {
		background: none;
		border: none;
		outline: none;
	}
	button, a, select {
		height: 100%;
		width: 1.5em;
		border-radius: inherit;
		cursor: pointer;
		opacity: 20%;
		&:hover {
			opacity: 100%;
		}
	}
	select {
		appearance: none;
		width: 2em;
	}
	input[type="search"] {
		flex-grow: 1;
		width: 0;
	}
}
</style>
