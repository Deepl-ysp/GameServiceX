<!-- BaseSlider.vue -->
<script lang="ts" setup>
import { computed } from "vue";

export interface sliderStyleVariables {
  color?: string;
  background?: string;
  shadowColor?: string;
  shadow?: string;
  borderRadius?: string;
  borderColor?: string;
  border?: string;
  padding?: string;
  margin?: string;
  transition?: string;
  opacity?: string;
  overflow?: string;
  trackHeight?: string;
  thumbSize?: string;
}

interface Props {
  value?: number;
  min?: number;
  max?: number;
  step?: number;
  isDisabled?: boolean;
  styleVariables?: sliderStyleVariables;
}

const props = withDefaults(defineProps<Props>(), {
  value: 0,
  min: 0,
  max: 100,
  step: 1,
  isDisabled: false,
  styleVariables: () => {
    const color = "#409eff";
    const background = "#e0e0e0";
    return {
      color,
      background,
      shadowColor: "rgba(0,0,0,0.2)",
      shadow: `0 0 4px rgba(0,0,0,0.2)`,
      borderRadius: "4px",
      borderColor: "#ccc",
      border: "none",
      padding: "0",
      margin: "0",
      transition: "all 0.2s ease",
      opacity: "1",
      overflow: "visible",
      trackHeight: "6px",
      thumbSize: "16px",
    };
  },
});

const emit = defineEmits<{
  (e: 'update:value', val: number): void;
}>();

const cssVars = computed(() => {
  const vars = props.styleVariables!;
  return {
    "--color": vars.color,
    "--background": vars.background,
    "--shadowColor": vars.shadowColor,
    "--shadow": vars.shadow,
    "--borderRadius": vars.borderRadius,
    "--borderColor": vars.borderColor,
    "--border": vars.border,
    "--padding": vars.padding,
    "--margin": vars.margin,
    "--transition": vars.transition,
    "--opacity": vars.opacity,
    "--overflow": vars.overflow,
    "--trackHeight": vars.trackHeight,
    "--thumbSize": vars.thumbSize,
  };
});

const onInput = (e: Event) => {
  const target = e.target as HTMLInputElement;
  const newVal = Number(target.value);
  emit('update:value', newVal);
};
</script>

<template>
  <input
    type="range"
    class="base-slider"
    :style="cssVars"
    :min="min"
    :max="max"
    :step="step"
    :value="value"
    :disabled="isDisabled"
    @input="onInput"
  />
</template>

<style lang="scss" scoped>
.base-slider {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: var(--trackHeight, 6px);
  border-radius: var(--borderRadius, 4px);
  outline: none;
  transition: var(--transition, all 0.2s ease);
  opacity: var(--opacity, 1);
  margin: var(--margin, 0);
  padding: var(--padding, 0);
  border: var(--border, none);
  box-shadow: var(--shadow, none);

  /* 滑块按钮 - WebKit */
  &::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: var(--thumbSize, 16px);
    height: var(--thumbSize, 16px);
    background: var(--color, #409eff);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: var(--shadow, 0 0 4px rgba(0,0,0,0.2));
    transition: var(--transition, all 0.2s ease);
    border: 1px solid var(--borderColor, #ccc);
  }

  /* 滑块按钮 - Firefox */
  &::-moz-range-thumb {
    width: var(--thumbSize, 16px);
    height: var(--thumbSize, 16px);
    background: var(--color, #409eff);
    border-radius: 50%;
    cursor: pointer;
    border: 1px solid var(--borderColor, #ccc);
    box-shadow: var(--shadow, 0 0 4px rgba(0,0,0,0.2));
  }

  /* 滑块进度条（填充部分）- WebKit */
  &::-webkit-slider-runnable-track {
    height: var(--trackHeight, 6px);
    background: var(--background, #e0e0e0);
    border-radius: var(--borderRadius, 4px);
  }

  &:disabled {
    opacity: calc(var(--opacity, 1) - 0.4);
    cursor: not-allowed;
  }

  &:hover {
    opacity: calc(var(--opacity, 1) + 0.1);
  }

  &:focus {
    outline: none;
    box-shadow: 0 0 0 2px var(--color, #409eff);
  }
}
</style>