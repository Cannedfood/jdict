import { onBeforeMount, onBeforeUnmount, ref } from 'vue';

export function useMatchMedia(query: string) {
	let result = ref<boolean>();

	const matcher = window.matchMedia(query);

	const update = () => {
		result.value = matcher.matches;
	}

	onBeforeMount(() => matcher.addEventListener('change', update));
	onBeforeUnmount(() => matcher.removeEventListener('change', update));

	update();

	return result;
}
