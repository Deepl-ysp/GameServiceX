<script lang="ts" setup>
import { computed, type CSSProperties } from "vue";

export interface CharacterSpriteStyleVariables {
  objectFit?: string;
  filter?: string;
  borderRadius?: string;
}

interface Props {
  src?: string;
  characterId?: string;
  illustrationNumber?: number;
  visible?: boolean;
  opacity?: number;
  styleVariables?: CharacterSpriteStyleVariables;
  mask?: boolean;
  maskColor?: string;
  r?: number;
}

const props = withDefaults(defineProps<Props>(), {
  src: '',
  characterId: '',
  illustrationNumber: 0,
  visible: true,
  opacity: 1,
  styleVariables: () => ({
    objectFit: 'contain',
    filter: 'none',
    borderRadius: '0',
  }),
  mask: false,
  maskColor: 'rgba(0, 0, 0, 0.3)',
  r: 0,
});

const imageSrc = computed(() => {
  if (props.src) return props.src;
  if (props.characterId && props.illustrationNumber) {
    return `/assets/characters/${props.characterId}/${props.illustrationNumber}.png`;
  }
  return '';
});

const cssVars = computed(() => {
  const vars = props.styleVariables!;
  return {
    '--sprite-object-fit': vars.objectFit,
    '--sprite-filter': vars.filter,
    '--sprite-border-radius': vars.borderRadius,
  };
});

const maskStyle = computed<CSSProperties>(() => {
  if (!props.mask || !imageSrc.value) return {};
  return {
    background: props.maskColor,
    maskImage: `url(${imageSrc.value})`,
    maskMode: 'alpha',
    maskRepeat: 'no-repeat',
    maskSize: 'contain',
    maskPosition: 'center',
    WebkitMaskImage: `url(${imageSrc.value})`,
    WebkitMaskMode: 'alpha',
    WebkitMaskRepeat: 'no-repeat',
    WebkitMaskSize: 'contain',
    WebkitMaskPosition: 'center',
  };
});
</script>

<template>
  <div
    class="sprite-controller"
    :style="{
      opacity: visible ? opacity : 0,
      transition: 'opacity 0.3s ease, transform 0.3s ease',
      transform: `rotate(${r}deg)`,
      transformOrigin: 'center center',
    }"
  >
    <img
      v-if="imageSrc"
      :src="imageSrc"
      :alt="characterId || 'character'"
      :style="cssVars"
      class="sprite-image"
    />
    <div v-else class="sprite-placeholder">
      {{ characterId || '?' }}
    </div>

    <div
      v-if="mask && imageSrc"
      class="sprite-mask"
      :style="maskStyle"
    ></div>
  </div>
</template>

<style lang="scss" scoped>
.sprite-controller {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: flex-end;
  justify-content: center;
  overflow: hidden;
}

.sprite-image {
  width: 100%;
  height: 100%;
  object-fit: var(--sprite-object-fit, contain);
  filter: var(--sprite-filter, none);
  border-radius: var(--sprite-border-radius, 0);
  display: block;
  user-select: none;
  pointer-events: none;
}

.sprite-placeholder {
  color: #999;
  font-size: 14px;
  background: rgba(0, 0, 0, 0.1);
  padding: 10px;
  border-radius: 4px;
  border: 1px dashed #ccc;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}

.sprite-mask {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}
</style>