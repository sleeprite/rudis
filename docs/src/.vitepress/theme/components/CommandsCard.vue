<template>
    <a class="commands-card">
        <span>{{ title }}</span>
        <p class="commands-card-ellipsis">{{ description }}</p>
    </a>
</template>

<script lang="ts" setup>
import { toRefs } from 'vue';

const props = defineProps<{ title: string, description: string }>()
const { title, description } = toRefs(props)
</script>

<style>
.commands-card {
    border:1px solid #2e2e32;
    display:block;
    width:100%;
    padding:13px;
    border-radius:8px; 
}
.commands-card-ellipsis {
  white-space: nowrap;      
  overflow: hidden;        
  text-overflow: ellipsis;  
  color:rgba(235, 235, 245, 0.6);
  font-size:12px;
}
</style>