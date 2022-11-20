import { onMounted, onUnmounted, ref } from 'vue';

export function onTabFocusChange(changed: (focused: boolean) => void) {
	const onFocus = () => changed(true);
	const onBlur  = () => changed(false);

	onMounted(() => {
		window.addEventListener('focus', onFocus);
		window.addEventListener('blur', onBlur);
	});
	onUnmounted(() => {
		window.removeEventListener('focus', onFocus);
		window.removeEventListener('blur', onBlur);
	});
}

export function useTabFocus() {
	const focus = ref(false);

	return focus;
}
