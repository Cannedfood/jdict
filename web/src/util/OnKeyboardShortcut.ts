import { onMounted, onUnmounted } from "vue";

function parseShortcut(shortcut: string) {
	const keys = shortcut.toLowerCase().split("+");
	return {
		key: keys.pop()!,
		ctrl: keys.includes('ctrl'),
		shift: keys.includes('shift'),
		alt: keys.includes('alt'),
	};
}

export function onKeyboardShortcut(desc: { [key: string]: () => boolean|void }) {
	const cb = (e: KeyboardEvent) => {
		for(const [k, v] of Object.entries(desc)) {
			for(const exp of k.split(',').map(e => e.trim())) {
				const { key, ctrl, shift, alt } = parseShortcut(exp);
				if(
					(key === e.key.toLowerCase()) &&
					(!ctrl || e.ctrlKey) &&
					(!shift || e.shiftKey) &&
					(!alt || e.altKey)
				)
				{
					if(v() !== false) {
						e.preventDefault();
						e.stopPropagation();
						return;
					}
				}
			}
		}
	};
	const self = {
		subscribed: false,
		subscribe: () => {
			if(!self.subscribed)
				document.addEventListener('keydown', cb);
		},
		unsubscribe: () => {
			if(self.subscribed)
				document.removeEventListener('keydown', cb);
		}
	}
	onMounted(() => self.subscribe());
	onUnmounted(() => self.unsubscribe());
	return self;
}
