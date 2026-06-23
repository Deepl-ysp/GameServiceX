<!-- TogglableSlider.vue -->
<script lang="ts" setup>
import BaseSlider from '@components/base/BaseSlider.vue';
import { type sliderStyleVariables } from '@components/base/BaseSlider.vue';
import BaseSwitch from '@components/base/BaseSwitch.vue';
import { type switchStyleVariables } from '@components/base/BaseSwitch.vue';

interface Props {
  isOn?: boolean;
  value?: number;
  min?: number;
  max?: number;
  isCN?: boolean;
  isDisabled?: boolean;
  sliderStyleVariables?: sliderStyleVariables;
  switchStyleVariables?: switchStyleVariables;
}

const props = withDefaults(defineProps<Props>(), {
  isOn: true,
  value: 50,
  min: 0,
  max: 100,
  isCN: true,
  isDisabled: false,
  sliderStyleVariables: () => ({}),
  switchStyleVariables: () => ({}),
});

const emit = defineEmits<{
  (e: 'update:value', val: number): void;
  (e: 'update:isOn', val: boolean): void;
}>();

const onValueUpdate = (val: number) => emit('update:value', val);
const onIsOnUpdate = (val: boolean) => emit('update:isOn', val);
</script>

<template>
  <div class="togglable-slider">
    <slot class="label"></slot>
    <BaseSlider
      :value="value"
      :min="min"
      :max="max"
      :is-disabled="isDisabled"
      :style-variables="sliderStyleVariables"
      @update:value="onValueUpdate"
    />
    <BaseSwitch
      :is-on="isOn"
      :is-cn="isCN"
      show-text
      :is-disabled="isDisabled"
      :style-variables="switchStyleVariables"
      @update:is-on="onIsOnUpdate"
    />
  </div>
</template>

<style lang="scss" scoped>
.togglable-slider {
  display: flex;
  align-items: center;
  gap: 12px;
  background-color: yellow;
  padding: 8px 12px;
}
</style>