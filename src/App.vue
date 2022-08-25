<script setup lang="ts">
import { ref, watch } from 'vue';
import SearchBar from './components/SearchBar.vue';

import { useRoute, useRouter } from 'vue-router'
import { computed } from '@vue/reactivity';
import Octicon from './components/Octicon.vue';
import { Entities } from './backend/jmdict';

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
	if(!last.startsWith('#')) return [];
	const words = [
		'kanji',
		...[1, 2, 3, 4, 5].map(n => 'jlpt' + n),
		...Object.keys(Entities),
	].map(w => '#' + w);
	return words.filter(w => w.startsWith(last) && !w.endsWith(last));
});

const home = computed(() => !route.name || route.name == 'home');

</script>

<template lang="pug">
nav
	.left
		a.btn(v-if="!home" href="/#/")
			Octicon(type="home")
	.right
	.search-bar-container.center(:class="{ home: home }")
		SearchBar(
			v-model="searchQuery"
			@send="router.push(`/search/${searchQuery}`)"
			:suggestions="suggestions"
		)
			a(@click="router.replace(`/kanji-grid/${searchQuery}`)") 漢
RouterView
</template>

<style lang="scss" scoped>
nav {
	display: grid;
	grid-template-columns: .5fr 2fr .5fr;
	grid-template-areas: "left center right";
	.left  {
		grid-area: left;
		text-align: left;
	}
	.right {
		grid-area: right;
		text-align: right;
	}
	.center {
		grid-area: center;
		text-align: center;
		width: 100%;
	}
}
.search-bar-container {
	margin-left: auto;
	margin-right: auto;
	transition: margin-top 100ms;
	&.home {
		margin-top: 40vh;
	}
}
</style>
