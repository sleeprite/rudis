<script setup lang="ts">
import { computed, onMounted, ref, toRefs } from 'vue'
import moment from 'moment'


// Mimic the <ClientOnly /> behavior to show custom text while rendering.
const show = ref(false)

onMounted(() => {
  show.value = true
})
</script>

<template>

</template>
