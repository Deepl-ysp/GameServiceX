<script lang="ts" setup>
import { computed, watch, onMounted } from "vue";

export interface CharLayerStyleVariables {
  width?: string;
  height?: string;
  transition?: string;
}

interface Props {
  x: number;
  y: number;
  z: number;
  visible?: boolean;
  zIndex?: number;
  styleVariables?: CharLayerStyleVariables;
}

const props = withDefaults(defineProps<Props>(), {
  x: 0,
  y: 0,
  z: 1.0,
  visible: true,
  zIndex: 1,
  styleVariables: () => ({
    width: '200px',
    height: '300px',
    transition: 'all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94)',
  }),
});

const containerWidth = computed(() => {
  const w = props.styleVariables?.width || '200px';
  return parseFloat(w);
});
const containerHeight = computed(() => {
  const h = props.styleVariables?.height || '300px';
  return parseFloat(h);
});

const actualWidth = computed(() => containerWidth.value * props.z);
const actualHeight = computed(() => containerHeight.value * props.z);

const rect = computed(() => {
  const leftPx = (props.x / 100) * window.innerWidth;
  const bottomPx = (props.y / 100) * window.innerHeight;
  return {
    left: leftPx,
    bottom: bottomPx,
    right: leftPx + actualWidth.value,
    top: bottomPx + actualHeight.value,
  };
});

function isCompletelyOutOfViewport() {
  const { left, right, top, bottom } = rect.value;
  return right < 0 || left > window.innerWidth || top < 0 || bottom > window.innerHeight;
}

let lastWarned = false;

function checkAndWarn() {
  if (!props.visible) return;
  const isOut = isCompletelyOutOfViewport();
  if (isOut && !lastWarned) {
    console.warn(`[CharLayer] 立绘完全超出视口 (x=${props.x}, y=${props.y}, z=${props.z})`);
    lastWarned = true;
  } else if (!isOut && lastWarned) {
    lastWarned = false;
  }
}

watch(
  [() => props.x, () => props.y, () => props.z, () => props.visible],
  checkAndWarn,
  { immediate: true }
);

onMounted(() => {
  window.addEventListener('resize', checkAndWarn);
  checkAndWarn();
});
</script>

<template>
  <div
    class="char-layer"
    :style="{
      left: x + '%',
      bottom: y + '%',
      transform: `scale(${z})`,
      transformOrigin: 'bottom left',
      opacity: visible ? 1 : 0,
      zIndex: zIndex,
      width: (styleVariables?.width || '200px'),
      height: (styleVariables?.height || '300px'),
      transition: (styleVariables?.transition || 'all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94)'),
    }"
    v-show="visible"
  >
    <slot></slot>
  </div>
</template>

<style lang="scss" scoped>
.char-layer {
  position: absolute;
  will-change: transform, opacity;
  pointer-events: none;
  display: flex;
  align-items: flex-end;
  justify-content: center;

  :deep(*) {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }
}
</style>