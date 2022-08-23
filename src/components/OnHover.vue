<script setup lang="ts">
import { ref } from 'vue';

const hovered = ref(false);
</script>

<template lang="pug">
span.on-hover(@mouseenter="hovered = true" @mouseleave="hovered = false")
	slot
	slot(v-if="hovered" name="when-hovered")
</template>
