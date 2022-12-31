import { onBeforeUnmount, onMounted } from "vue";

export function useInterval(millis: number, callback: () => void) {
    let delay: number;
    onMounted(() => delay = setInterval(callback, millis));
    onBeforeUnmount(() => clearInterval(delay));
}
