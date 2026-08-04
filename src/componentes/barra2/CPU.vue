<template>
  <SystemItem icono="/cpu.png" nombre="CPU" :porcentaje="data" />
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import SystemItem from '../barra2/Sistema.vue';
import { useSystemMonitor } from '../../composables/useSystemMonitor';

const getCpu = async () => {
  return await invoke('get_cpu_percentage_int');
};

const { data } = useSystemMonitor(getCpu, 3000);
</script>