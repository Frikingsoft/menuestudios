<template>
  <SystemItem 
    icono="/disco.png" 
    nombre="DISCO" 
    :porcentaje="porcentaje"
    :umbrales="{ low: 50, medium: 70, high: 85 }"
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
    porcentaje.value = await invoke('get_disk_percentage_int');
  } catch (error) {
    console.error('Error al obtener disco:', error);
  }
};

onMounted(() => {
  actualizar();
  intervalId = setInterval(actualizar, 5000);
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});
</script>