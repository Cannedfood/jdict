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

const home = computed(() => !route.name || route.name == 'home');

</script>

<template lang="pug">
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
	background-color: #242424;

	// Show in center of screen when on home page
	transition: transform 80ms ease-in-out;
	&.home { transform: translateY(40vh); }
}
</style>
