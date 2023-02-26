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
function focus() {
	searchInput.value?.focus();
	(navigator as any).virtualKeyboard?.show();
}
function blur() { searchInput.value?.blur(); }
function send() {
	searchInput.value?.select();
	emit('send', props.modelValue);
}

onKeyboardShortcut({
	'alt+f,alt+u': () => {
		if(searchInput.value === document.activeElement)
			return false;
		focus();
	}
});

onScrolledUsingTouch(() => searchInput.value?.blur());

onTabFocusChange(hasFocus => {
	if(hasFocus)
		setTimeout(() => focus(), 100); // Need a timeout so the click event (from clicking into the tab) and doesn't immediately steal the focus back
	else
		blur();
});

const hasSuggestions = ref(false);

</script>

<template lang="pug">
.search-bar(:class="{ 'has-suggestions': hasSuggestions }")
	input(type="search" ref="searchInput"
		placeholder="Search..."

		:value="modelValue"
		@keydown.enter.prevent="emit('update:modelValue', targetValue($event).toLowerCase()); send()"

		onfocus="this.select()"

		autofocus
		autocomplete="off"
		autocapitalize="none"
	)
	.buttons
		slot
	.suggestions(v-if="hasSuggestions")
		div
			| Hello
			kbd.float-right tab
</template>

<style lang="scss" scoped>
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
	&.has-suggestions {
		border-bottom-left-radius: 0;
		border-bottom-right-radius: 0;
	}

	input[type="search"] {
		font-size: inherit;
		margin: 0;
		padding-block: .1em;
		padding-inline: .1em;
		border-radius: 0;
		flex-grow: 1;
		width: 0;
	}
	.buttons {
		input, button, select, a {
			background: none;
			border: none;
			outline: none;
		}
		button, a, select {
			height: 100%;
			width: 1.5em;
			border-radius: inherit;
			text-decoration: none;
			cursor: pointer;
			color: var(--text-muted2);
			transition: color 200ms;
			&:hover {
				color: var(--text1);
			}
		}
	}

	.suggestions {
		position: absolute;
		top: 100%;
		left: calc(-1 * var(--outline-width));
		right: calc(-1 * var(--outline-width));
		z-index: 1;

		background: var(--background2);
		border-bottom-left-radius: .5em;
		border-bottom-right-radius: .5em;
		padding-block: .2em;
		border: var(--outline-width) solid var(--neutral1);
		border-top: calc(0.5 * var(--outline-width)) solid var(--neutral2);
	}
}
</style>
