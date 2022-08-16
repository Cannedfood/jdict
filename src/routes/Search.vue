<script setup lang="ts">
import { inject, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import type { SearchService, SearchResult } from '../backend/search'
import Result from '../components/Result.vue';
import Spinner from '../components/Spinner.vue';
import { throttle } from 'lodash'

const searchService = inject<SearchService>('search-service')!;
const route = useRoute();

const search = ref<SearchResult>();
const searchInProgress = ref(0);

watch(
	() => route.params.query,
	async() => {
		if(route.params.query) {
			console.log("Search " + route.params.query)
			try {
				searchInProgress.value++;
				search.value = undefined;
				search.value = await searchService.search(route.params.query as string, { take: 10 });
			}
			finally { searchInProgress.value--; }
		}
	},
	{ immediate: true }
);

const searchMore = throttle(async() => {
	if(search.value && !searchInProgress.value) {
		console.log("Load more");
		try {
			searchInProgress.value++;
			search.value = await searchService.searchMore(search.value);
			console.log(`-> Loaded ${search.value.results.length} of ${search.value.resultsTotal}`)
		}
		finally { searchInProgress.value--; }
	}
}, 500);

async function scrollHandler() {
	const notAllResultsLoaded = search.value && search.value.results.length < search.value.resultsTotal;
	const atBottom = window.innerHeight + window.pageYOffset >= document.body.offsetHeight;
	if (notAllResultsLoaded && atBottom) {
		searchMore();
	}
}
onMounted(() => window.addEventListener('scroll', scrollHandler));
onUnmounted(() => window.removeEventListener('scroll', scrollHandler));

</script>

<template lang="pug">
.stats(v-if="search") {{search.resultsTotal}} Results ({{search.time}})
.container
	.results(v-if="search")
		Result(v-for="entry of search.results" :entry="entry")
	Spinner(v-if="searchInProgress")
	hr
</template>

<style lang="scss">
.stats {
	color: #FFF3;
	// text-align: right;
	margin-block: 1em;
	font-size: .8em;
}
</style>
