<script lang="ts" setup>
import { RouterView } from 'vue-router';
import "@styles/dialog.css"
</script>

<template>
  <div class="view">
    <RouterView></RouterView>
  </div>
</template>
<style scoped>
.view{
  position: relative;
  width: 100vw;
  height: 100vh;
}
</style>