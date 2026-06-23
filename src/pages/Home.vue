<script lang="ts" setup>
import { getImage } from '@utils/image';
import { onMounted, ref } from 'vue';

const url = ref<string>();

onMounted(async()=>{
    url.value = (await getImage("image/Character/Abydos/CountermeasuresCommittee/NonomiIzayoi/01.webp")).data
})

</script>

<template>
    <div>
        <img :src="url" alt="">
    </div>
</template>

