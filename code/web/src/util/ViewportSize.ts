import { onMounted, onUnmounted, reactive } from "vue";

export function useViewportSize() {
    const result = reactive({
        width: 0,
        height: 0
    });
    
    const updateSize = () => {
        result.width = window.innerWidth;
        result.height = window.innerHeight;
    };
    
    onMounted(() => {
        updateSize();
        window.addEventListener('resize', updateSize);
    });
    
    onUnmounted(() => {
        window.removeEventListener('resize', updateSize);
    });

    return result;
}