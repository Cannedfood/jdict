import { onMounted, onUnmounted } from "vue";

export function onKeyboardShortcut(desc: { [key: string]: () => boolean|void }) {
    const cb = (e: KeyboardEvent) => {
        for(const [k, v] of Object.entries(desc)) {
            for(const exp of k.split(',').map(e => e.trim())) {
                if(exp.toLowerCase() === e.key.toLowerCase()) {
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