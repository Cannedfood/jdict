import { onMounted, onUnmounted } from 'vue';

export interface Options {
	padding: number,
}

export function onScrolledToBottom(cb: () => void, options: Partial<Options> = {}) {
	const opt = Object.assign(
		<Options>{
			padding: 150
		},
		options
	);

	function scrollHandler() {
		const atBottom = window.innerHeight + window.pageYOffset + opt.padding >= document.body.offsetHeight;
		if (atBottom)
			cb();
	}
	onMounted(() => window.addEventListener('scroll', scrollHandler));
	onUnmounted(() => window.removeEventListener('scroll', scrollHandler));
}
