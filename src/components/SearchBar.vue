<script setup lang="ts">
import { nextTick, onMounted, ref } from 'vue';
import SearchBarSuggestions from './SearchBarSuggestions.vue';

const props = defineProps<{
	modelValue: string,
	suggestions: string[],
}>();
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void,
  (e: 'send', value: string): void
}>()

function targetValue(e: any) { return e.target.value; }

const searchInput = ref<HTMLInputElement | null>(null);
onMounted(() => nextTick(() => searchInput.value?.focus()))

function send() {
	if(searchInput.value?.value !== '')
		searchInput.value?.blur();
	emit('send', props.modelValue);
}

const suggestionBox = ref<typeof SearchBarSuggestions>(undefined!);

</script>

<template lang="pug">
.search-bar
	input(
		ref="searchInput"
		type="search"
		placeholder="Search..."
		:value="modelValue"
		autocomplete="off"
		@input="emit('update:modelValue', targetValue($event).toLowerCase())"
		@keydown.enter.prevent="send()"
		@keydown.up.prevent="suggestionBox.suggestionUp()"
		@keydown.down.prevent="suggestionBox.suggestionDown()"
		@keydown.tab.prevent="suggestionBox.acceptSuggestion()"
		onfocus="this.select();"
	)
	slot
	SearchBarSuggestions(
		ref="suggestionBox"
		:suggestions="suggestions"
		@accept="emit('update:modelValue', $event)"
	)
</template>

<style lang="scss">

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

	.suggestions {
		position: absolute;
		top: 100%;
		left: .5em;
		right: .5em;
		border-bottom-left-radius: .2em;
		border-bottom-right-radius: .2em;
		border: 1px solid purple;
		border-top: none;
		background: #242424;

		text-align: left;
	}
	&:not(:focus-within) {
		.suggestions {
			display: none;
		}
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
