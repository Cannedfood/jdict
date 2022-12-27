import { onMounted, onUnmounted } from 'vue';

export function onScrolledUsingTouch(action: () => void) {
	const onScrolled = () => {
		action();
	}
	onMounted(() => window.addEventListener('touchmove', onScrolled));
	onUnmounted(() => window.removeEventListener('touchmove', onScrolled));
}
