// composables/useSystemMonitor.js
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export function useSystemMonitor(fn, intervalo = 3000) {
  const data = ref(0);
  let rafId = null;
  let lastUpdate = 0;
  let isMounted = true;

  const actualizar = async () => {
    if (!isMounted) return;
    
    try {
      data.value = await fn();
    } catch (error) {
      console.error('Error al actualizar:', error);
    }
  };

  const loop = (timestamp) => {
    if (!isMounted) return;
    
    if (timestamp - lastUpdate >= intervalo) {
      actualizar();
      lastUpdate = timestamp;
    }
    
    rafId = requestAnimationFrame(loop);
  };

  const iniciar = () => {
    isMounted = true;
    actualizar();
    lastUpdate = performance.now();
    rafId = requestAnimationFrame(loop);
  };

  const detener = () => {
    isMounted = false;
    if (rafId) {
      cancelAnimationFrame(rafId);
      rafId = null;
    }
  };

  onMounted(iniciar);
  onUnmounted(detener);

  return { data };
}