<script setup lang="ts">
import { debounce } from 'lodash';
import { nextTick, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useSearch } from '../backend/search';

const search = useSearch();

const results = ref([] as string[]);
const searchTerm = ref('');
const route = useRoute();
const router = useRouter();

const runSearch = debounce(async(term: string) => {
	if(term.length > 2)
		results.value = await search.searchKanji(searchTerm.value);
	else
		results.value = [];
}, 100);

watch(
	() => searchTerm.value,
	() => {
		if(searchTerm.value.length < 2)
			results.value = [];
		else
			runSearch(searchTerm.value);
	}
);

const hovered = ref(undefined as string|undefined);

const kanjiSearchInput = ref<HTMLInputElement|null>(null);
function addKanji(kanji: string) {
	router.replace(route.fullPath + kanji);
	nextTick(() => {
		kanjiSearchInput.value?.focus();
	});
}

</script>

<template lang="pug">
.container
	input(
		ref="kanjiSearchInput"
		type="search"
		v-model="searchTerm"
		autofocus
	)
	.kanji
		.entry(
			v-for="r in results"
			@click="searchTerm = ''; addKanji(r);"
			@mouseenter="hovered = r"
			@mouseleave="hovered = undefined"
		) {{r}}
</template>

<style lang="scss" scoped>
input {
	margin: .5em;
	display: block;
	font-size: 2em;
	background: #0002;
	padding: .2em;
	border-radius: .2em;
	width: 100%;
	margin-inline: auto;

	transition: 400ms border-color;
	border: 1px solid transparent;
	&:focus {
		border-color: purple;
	}
}
.kanji {
	display: flex;
	flex-flow: wrap row;
	.entry {
		cursor: pointer;
		padding: .5em;
		margin: .1em;
		background: #0002;
		&:hover {
			background: #8882;
			font-weight: bold;
		}
	}
}
</style>
