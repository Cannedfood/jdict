<script setup lang="ts">
import { debounce } from 'lodash';
import { ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useSearch } from '../backend/search';

const search = useSearch();

const results = ref([] as string[]);
const searchTerm = ref('');
const route = useRoute();

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

</script>

<template lang="pug">
.container
	input(type="search" v-model="searchTerm")
	.kanji
		.entry(
			v-for="r in results"
			@click="route.params.query += r; searchTerm = '';"
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
