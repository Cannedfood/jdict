<script setup lang="ts">
import { inject, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import type { SearchService, SearchResult } from '../backend/search'
import Entry from '../components/Entry.vue';
import Spinner from '../components/Spinner.vue';
import { throttle } from 'lodash'
import { onScrolledToBottom } from '../util/OnScrolledToBottom';
import { computed } from '@vue/reactivity';
import KanjiInfo from '@/components/KanjiInfo.vue';

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
		try {
			searchInProgress.value++;
			search.value = await searchService.searchMore(search.value);
			console.log(`...loaded ${search.value.results.length} of ${search.value.resultsTotal}`)
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
.stats(v-if="search") {{search.resultsTotal}} Results (server: {{search.time}}, client: {{search.clientTime}})
.container
	.kanji-infos
		KanjiInfo(
			v-if="search && search.kanji"
			v-for="kanji of search.kanji"
			:kanji="kanji"
		)
	.results(:class="{ visible: search?.results?.length }")
		Entry(v-for="entry of search?.results" :entry="entry")
		Spinner(v-if="searchInProgress")
		hr(v-if="allResultsLoaded")
</template>

<style lang="scss" scoped>
@media (min-width: 1600px) {
	.kanji-infos {
		position: absolute;
		left: 0;
		max-width: 20em;
	}
}

@keyframes appear {
	from { opacity: 0; }
	to   { opacity: 1; }
}
.results {
	opacity: 0;
	&.visible {
		animation: appear 0.1s ease-in-out;
		opacity: 1;
	}
}
.stats {
	color: #888A;
	// text-align: right;
	margin-block: 1em;
	margin-inline: .6em;
	font-size: .8em;
}
</style>
