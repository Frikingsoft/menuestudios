<template>
  <SystemItem 
    icono="/memoria.png" 
    nombre="RAM" 
    :porcentaje="data"
    :umbrales="{ low: 50, medium: 75, high: 90 }"
  />
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import SystemItem from '../barra2/Sistema.vue';
import { useSystemMonitor } from '../../composables/useSystemMonitor';

const getRam = async () => {
  const result = await invoke('get_ram_percentage');
  return Math.round(result);
};

const { data } = useSystemMonitor(getRam, 3000);
</script>