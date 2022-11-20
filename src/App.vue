<script setup lang="ts">
import { ref, watch } from 'vue';
import SearchBar from './components/SearchBar.vue';

import { useRoute, useRouter } from 'vue-router'
import { computed } from '@vue/reactivity';
import BackgroundImage from './components/BackgroundImage.vue';

const route = useRoute();
const router = useRouter();

const searchQuery = ref('');
watch(
	() => route.params.query,
	() => searchQuery.value = route.params.query as string ?? ''
);

const home = computed(() => !route.name || route.name == 'home');

</script>

<template lang="pug">
BackgroundImage(:visible="home")
nav(:class="{ home: home }")
	SearchBar(
		v-model="searchQuery"
		@send="router.push(`/search/${searchQuery}`)"
	)
		a(@click="router.replace(`/kanji-grid/${searchQuery}`)") 漢
RouterView
</template>

<style lang="scss" scoped>
nav {
	// Stick to top
	position: sticky;
	top: 0;
	z-index: 2;

	// Center search bar vertically
	display: flex;
	flex-flow: nowrap row;
	align-items: center;

	// Padding
	padding-block: .1em;

	// Show in center of screen when on home page
	transition: transform var(--home-transition-time) ease-out;
	min-height: 0;
	&.home {
		transform: translateY(40vh);
		background-color: var(--layer1);
		min-height: 20vh;
	}
}
</style>
