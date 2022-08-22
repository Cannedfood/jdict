<script setup lang="ts">
import { ref, watch } from 'vue';
import SearchBar from './components/SearchBar.vue';

import { useRoute, useRouter } from 'vue-router'
import { computed } from '@vue/reactivity';

const route = useRoute();
const router = useRouter();

const searchQuery = ref('');
watch(
	() => route.params.query,
	() => searchQuery.value = route.params.query as string ?? ''
);
const suggestions = computed(() => {
	if(searchQuery.value.endsWith(' ')) return [];
	const tokens = searchQuery.value.split(' ').filter(t => t);
	if(!tokens.length) return [];
	const last = tokens[tokens.length - 1];
	const words = ['#kanji', '#name', '#fun'];
	return words.filter(w => w.startsWith(last) && !w.endsWith(last));
});

const home = computed(() => !route.name || route.name == 'home');

</script>

<template lang="pug">
//- .settings-bar
//- 	button theme:dark
//- 	select
//- 		option [EN]
//- 		option [DE]
//- 		option [RU]
nav
	.right
	.search-bar-container(:class="{ center: home }")
		SearchBar(
			v-model="searchQuery"
			@send="router.push(`/search/${searchQuery}`)"
			:suggestions="suggestions"
		)
			a(@click="router.replace(`/kanji-grid/${searchQuery}`)") 漢
RouterView
</template>

<style lang="scss">
nav {
	.left { float: left; }
	.right { float: right; }
	.search-bar-container {
		flex-grow: 1;
	}
}
.search-bar-container {
	margin-left: auto;
	margin-right: auto;
	transition: margin-top 100ms;
	&.center {
		margin-top: 40vh;
	}
}
</style>
