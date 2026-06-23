<script setup lang="ts">
import { computed } from "vue";

interface styleVariables {
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
}

interface Props {
  isDisabled?: boolean;
  styleVariables?: styleVariables;
}

const props = withDefaults(defineProps<Props>(), {
  isDisabled: false,
  styleVariables: () => {
    const shadowColor = "#c9c9c9";
    const borderColor = "#c0c0c0"
    return {
      color: "#00b3ff",
      background: "#f5f5f5",
      shadowColor: "#c9c9c9",
      shadow: `0px 0px 5px ${shadowColor}`,
      borderRadius: "500px",
      borderColor: "#c0c0c0",
      border: `1px solid ${borderColor}`,
      padding: "0.35rem 0.5rem 0.3rem 0.5rem",
      margin: "0",
      transition: "all 0.2s ease-in-out",
      opacity: "0.8",
      overflow: "hidden",
    }
  }
});

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
    "--overflow": vars.overflow
  }
});
</script>

<template>
  <button :style="cssVars" class="BaseButton" :disabled="isDisabled"">
    <slot />
    <div class=" lien"></div>
  </button>
</template>

<style lang="scss" scoped>
.BaseButton {
  cursor: pointer;
  color: var(--color);
  background-color: var(--background);
  box-shadow: var(--shadow);
  border: var(--border);
  border-radius: var(--borderRadius);
  padding: var(--padding);
  margin: var(--margin);
  transition: var(--transition);
  opacity: var(--opacity);
  overflow: var(--overflow);

  &:hover {
    opacity: calc(var(--opacity) + 0.2);
    transform: scale(1.05);

    .lien {
      width: 100%;
      transform: translateX(0%);
      transform: scale(1.05);
    }
  }

  &:active {
    opacity: calc(var(--opacity) - 0.2);
    transform: scale(0.95);
    box-shadow: 0 0 0 0.5rem #0000004f;
  }

  &:disabled {
    opacity: calc(var(--opacity) - 0.4);
    cursor: not-allowed;
    transform: none !important;

    .lien {
      width: 0%;
      height: 0%;
      transform: translateX(-100%);
    }
  }

  &:focus {
    outline: none;
    border: 1px solid var(--color);
  }
}

.lien {
  background-color: var(--color);
  width: 0%;
  height: 1px;
  transform: translateX(100%);
  transition: all 0.1s ease-in-out;
}
</style>
