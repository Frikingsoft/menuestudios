<template>
  <div class="system-item">
    <img :src="icono" class="system-icon" :alt="nombre" />
    <span class="system-text" :class="{ critical: isCritical }" :style="{ color: textColor }">
      {{ porcentaje }}%
    </span>
  </div>
</template>

<script setup>
import { computed, ref, onMounted, onUnmounted } from 'vue';

const props = defineProps({
  icono: String,
  nombre: String,
  porcentaje: Number,
  umbrales: {
    type: Object,
    default: () => ({
      low: 40,
      medium: 70,
      high: 90
    })
  }
});

const textColor = computed(() => {
  const p = props.porcentaje;
  const { low, medium, high } = props.umbrales;
  
  if (p < low) return '#4ade80';
  if (p < medium) return '#fbbf24';
  if (p < high) return '#fb923c';
  return '#f87171';
});

const isCritical = computed(() => props.porcentaje >= props.umbrales.high);
</script>

<style scoped>
.system-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.system-icon {
  width: 40px;
  height: 40px;
  object-fit: contain;
  filter: drop-shadow(0 0 8px rgba(131, 100, 255, 0.2));
  transition: all 0.3s ease;
  flex-shrink: 0;
}

.system-text {
  font-size: 1rem;
  font-weight: bold;
  font-family: 'Courier New', monospace;
  min-width: 35px;
  letter-spacing: 0.5px;
  transition: color 0.5s ease;
  text-shadow: v-bind('`0 0 10px ${textColor}33`');
}

.system-text.critical {
  animation: pulse-text 1s ease-in-out infinite;
}

@keyframes pulse-text {
  0%, 100% { 
    opacity: 1;
    transform: scale(1);
  }
  50% { 
    opacity: 0.7;
    transform: scale(1.05);
  }
}

@media (max-width: 768px) {
  .system-icon {
    width: 20px;
    height: 20px;
  }
  
  .system-text {
    font-size: 0.7rem;
    min-width: 28px;
  }
}

@media (max-width: 480px) {
  .system-text {
    min-width: 24px;
    font-size: 0.6rem;
  }
}
</style>