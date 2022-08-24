<script setup lang="ts">
import { inject, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import type { SearchService, SearchResult } from '../backend/search'
import Entry from '../components/Entry.vue';
import Spinner from '../components/Spinner.vue';
import { throttle } from 'lodash'
import { onScrolledToBottom } from '../util/OnScrolledToBottom';
import { computed } from '@vue/reactivity';

const searchService = inject<SearchService>('search-service')!;
const route = useRoute();

const search = ref<SearchResult>();
const searchInProgress = ref(0);

const allResultsLoaded = computed(() => search.value && search.value.results.length == search.value.resultsTotal);

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

async function searchMore() {
	if(search.value && !allResultsLoaded.value && !searchInProgress.value) {
		console.log("Load more");
		try {
			searchInProgress.value++;
			search.value = await searchService.searchMore(search.value);
			console.log(`-> Loaded ${search.value.results.length} of ${search.value.resultsTotal}`)
		}
		finally { searchInProgress.value--; }
	}
}

onScrolledToBottom(
	throttle(searchMore, 500, { leading: true, trailing: false }),
	{ padding: 1200 }
)

</script>

<template lang="pug">
.stats.float-left(v-if="search") {{search.resultsTotal}} Results (server: {{search.time}}, client: {{search.clientTime}})
.container
	.results(v-if="search")
		Entry(v-for="entry of search.results" :entry="entry")
	Spinner(v-if="searchInProgress")
	hr(v-if="allResultsLoaded")

</template>

<style lang="scss" scoped>
.stats {
	color: #FFF3;
	// text-align: right;
	margin-block: 1em;
	font-size: .8em;
}
</style>
