<script setup lang="ts">
import { onKeyboardShortcut } from '@/util/OnKeyboardShortcut';
import { onScrolledUsingTouch } from '@/util/OnScrolledUsingTouch';
import { onTabFocusChange } from '@/util/OnTabFocusChange';
import { ref } from 'vue';

const props = defineProps<{
	modelValue: string,
}>();
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void,
  (e: 'send', value: string): void
}>()

function targetValue(e: any) { return e.target.value; }

const searchInput = ref<HTMLInputElement | null>(null);

onKeyboardShortcut({
	'alt+f,alt+u': () => {
		if(searchInput.value === document.activeElement)
			return false;
		searchInput.value?.focus();
	}
});

onScrolledUsingTouch(() => searchInput.value?.blur());

onTabFocusChange(focus => {
	if(focus) {
		// Need a timeout so the click event (from clicking into the tab) and doesn't immediately steal the focus back
		setTimeout(() => searchInput.value?.focus(), 50);
	}
	else {
		searchInput.value?.blur();
	}
});

function send() {
	searchInput.value?.select();
	emit('send', props.modelValue);
}

</script>

<template lang="pug">
.search-bar
	input(type="search" ref="searchInput"
		placeholder="Search..."

		:value="modelValue"
		@keydown.enter.prevent="emit('update:modelValue', targetValue($event).toLowerCase()); send()"

		onfocus="this.select()"

		autofocus
		autocomplete="off"
		autocapitalize="none"
	)
	slot
</template>

<style lang="scss">

.search-bar {
	display: flex;
	flex-flow: nowrap row;

	position: relative;

	box-sizing: border-box;
	width: 14cm;
	max-width: calc(100vw - .5em);
	margin-block: .2em;
	margin-inline: auto;

	font-size: 2em;

	background: var(--background2);
	border-radius: .5em;
	padding-inline: .3em;
	padding-block: .2em;

	transition: 400ms border-color;
	border: var(--outline-width) solid var(--neutral1);
	&:focus-within {
		border-color: var(--accent1);
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
		color: var(--text-muted2);
		transition: color 200ms;
		&:hover {
			color: var(--text1);
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
