<!-- BaseSwitch.vue -->
<script lang="ts" setup>
import { computed } from "vue";

export interface switchStyleVariables {
  color?: string;          // 开启状态背景色
  background?: string;     // 关闭状态背景色
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
  // 额外
  switchWidth?: string;    // 开关宽度
  switchHeight?: string;   // 开关高度
  knobSize?: string;       // 按钮大小
  knobColor?: string;      // 按钮颜色
}

interface Props {
  isOn?: boolean;
  isCN?: boolean;
  showText?: boolean;
  isDisabled?: boolean;
  styleVariables?: switchStyleVariables;
}

const props = withDefaults(defineProps<Props>(), {
  isOn: true,
  isCN: true,
  showText: true,
  isDisabled: false,
  styleVariables: () => {
    const color = "#409eff";
    const background = "#ccc";
    return {
      color,
      background,
      shadowColor: "rgba(0,0,0,0.3)",
      shadow: "0 1px 3px rgba(0,0,0,0.3)",
      borderRadius: "999px",
      borderColor: "transparent",
      border: "none",
      padding: "0",
      margin: "0",
      transition: "all 0.2s ease",
      opacity: "1",
      overflow: "visible",
      switchWidth: "44px",
      switchHeight: "22px",
      knobSize: "18px",
      knobColor: "white",
    };
  },
});

const emit = defineEmits<{
  (e: 'update:isOn', val: boolean): void;
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
    "--switchWidth": vars.switchWidth,
    "--switchHeight": vars.switchHeight,
    "--knobSize": vars.knobSize,
    "--knobColor": vars.knobColor,
  };
});

const toggle = () => {
  if (props.isDisabled) return;
  emit('update:isOn', !props.isOn);
};

const statusText = computed(() => {
  if (props.isCN) return props.isOn ? '开启' : '关闭';
  return props.isOn ? 'On' : 'Off';
});
</script>

<template>
  <div class="base-switch-wrapper" :style="cssVars">
    <button
      class="base-switch"
      :class="{ on: isOn, off: !isOn }"
      :disabled="isDisabled"
      @click="toggle"
    >
      <span class="switch-knob"></span>
    </button>
    <span v-if="showText" class="status-text">{{ statusText }}</span>
  </div>
</template>

<style lang="scss" scoped>
.base-switch-wrapper {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  opacity: var(--opacity, 1);
  margin: var(--margin, 0);
  padding: var(--padding, 0);
  transition: var(--transition, all 0.2s ease);
}

.base-switch {
  position: relative;
  width: var(--switchWidth, 44px);
  height: var(--switchHeight, 22px);
  border: var(--border, none);
  border-radius: var(--borderRadius, 999px);
  background: var(--background, #ccc);
  cursor: pointer;
  transition: background 0.2s ease;
  padding: 0;
  outline: none;
  box-shadow: var(--shadow, 0 1px 3px rgba(0,0,0,0.3));

  &.on {
    background: var(--color, #409eff);
  }

  &.off {
    background: var(--background, #ccc);
  }

  .switch-knob {
    position: absolute;
    top: 50%;
    left: 2px;
    transform: translateY(-50%);
    width: var(--knobSize, 18px);
    height: var(--knobSize, 18px);
    border-radius: 50%;
    background: var(--knobColor, white);
    transition: left 0.2s ease;
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
  }

  &.on .switch-knob {
    left: calc(100% - var(--knobSize, 18px) - 2px);
  }

  &:hover {
    opacity: calc(var(--opacity, 1) + 0.1);
  }

  &:active .switch-knob {
    transform: translateY(-50%) scale(0.9);
  }

  &:disabled {
    opacity: calc(var(--opacity, 1) - 0.4);
    cursor: not-allowed;
    transform: none !important;
  }

  &:focus-visible {
    outline: 2px solid var(--color, #409eff);
    outline-offset: 2px;
  }
}

.status-text {
  font-size: 14px;
  color: var(--color, #333);
  user-select: none;
  transition: var(--transition, all 0.2s ease);
}
</style>