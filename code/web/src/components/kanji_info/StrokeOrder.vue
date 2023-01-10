<script setup lang="ts">
import { allPaths } from '@/backend/kanjivg';
import type { Kanji } from '@/backend/kanjivg';
import { useInterval } from '@/util/UseInterval';
import { computed, ref } from 'vue';
import { makeAbsolute, parseSVG } from 'svg-path-parser';
import { MoveToCommand, LineToCommand, CurveToCommand, EllipticalArcCommand } from 'svg-path-parser';

const props = defineProps<{
  kanjivg: Kanji
}>();

const strokes = computed(() => allPaths(props.kanjivg));
const strokePoints = computed(() => (
    strokes.value.map(
        p => (
            (
                makeAbsolute(parseSVG(p.path))
                .filter(c => (
                    c.command == 'moveto' ||
                    c.command == 'lineto' ||
                    c.command == 'curveto' ||
                    c.command == 'elliptical arc' ||
                    c.command == 'quadratic curveto' ||
                    c.command == 'smooth curveto' ||
                    c.command == 'smooth quadratic curveto'
                )) as
                Array<MoveToCommand | LineToCommand | CurveToCommand | EllipticalArcCommand>
            )
            .map(c => [c.x, c.y])
        )
    )
));
// const strokeTextPositions = computed(() => strokePoints.value.map(points => points[points.length - 1]));

const stroke = ref(0);
useInterval(250, () => stroke.value = (stroke.value + 1) % strokes.value.length);

</script>

<template lang="pug">
svg(view-box="0 0 109 109")
    path(
        v-for="s in strokes" 
        :d="s.path"
        stroke="gray"
        fill="none"
        stroke-width="1.5"
    )
    path(
        v-for="s, i in strokes.slice(0, stroke + 1)"
        :class="{ 'animate-dashed': i == stroke }"
        :d="s.path"
        stroke="white"
        fill="none"
        stroke-width="1.5"
    )
    circle(
        :cx="strokePoints[stroke][0][0]"
        :cy="strokePoints[stroke][0][1]"
        r="3"
        fill="red"
    )
</template>

<style lang="scss" scoped>
.animate-dashed {
    stroke-dasharray: 100;
    animation: dash .1s linear forwards;
}
@keyframes dash {
    from { stroke-dashoffset: 100; }
    to { stroke-dashoffset: 0; }
}
</style>
