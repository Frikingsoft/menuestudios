<template>
  <SystemItem 
    icono="/sonido.png" 
    nombre="SONIDO" 
    :porcentaje="porcentaje"
    :umbrales="{ low: 30, medium: 60, high: 85 }"
  />
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from '@tauri-apps/api/core';
import SystemItem from '../barra2/Sistema.vue';

const porcentaje = ref(0);
let intervalId = null;

const actualizar = async () => {
  try {
    const vol = await invoke('get_volume');
    porcentaje.value = vol;
  } catch (error) {
    console.error('Error al obtener sonido:', error);
  }
};

onMounted(() => {
  actualizar();
  intervalId = setInterval(actualizar, 200);
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});
</script>