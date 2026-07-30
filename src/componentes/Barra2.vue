<template>
    <div class="ram-container">
        <span class="ram-text">{{ percentage }}%</span>
        <div class="ram-bar">
            <div 
                class="ram-fill" 
                :style="{ width: percentage + '%' }"
                :class="getColorClass()"
            ></div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from '@tauri-apps/api/core';

const percentage = ref(0);

const getRam = async () => {
    try {
        const result = await invoke('get_ram_percentage');
        percentage.value = Math.round(result);
    } catch (error) {
        console.error('Error al obtener RAM:', error);
    }
};

const getColorClass = () => {
    if (percentage.value < 50) return 'low';
    if (percentage.value < 75) return 'medium';
    if (percentage.value < 90) return 'high';
    return 'critical';
};

onMounted(() => {
    getRam();
    const interval = setInterval(getRam, 3000);
    onUnmounted(() => clearInterval(interval));
});
</script>

<style scoped>
.ram-container {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 12px;
    background: rgba(131, 100, 255, 0.05);
    border: 1px solid rgba(131, 100, 255, 0.15);
    border-radius: 8px;
    box-shadow: 0 0 20px rgba(131, 100, 255, 0.05);
}

.ram-text {
    color: #d4c4ff;
    font-size: 0.8rem;
    font-weight: bold;
    font-family: 'Courier New', monospace;
    min-width: 40px;
    text-shadow: 0 0 10px rgba(131, 100, 255, 0.3);
}

.ram-bar {
    width: 60px;
    height: 4px;
    background: rgba(131, 100, 255, 0.15);
    border-radius: 2px;
    overflow: hidden;
}

.ram-fill {
    height: 100%;
    border-radius: 2px;
    transition: width 0.5s ease;
}

.ram-fill.low {
    background: linear-gradient(90deg, #4ade80, #22d3ee);
    box-shadow: 0 0 10px rgba(74, 222, 128, 0.3);
}

.ram-fill.medium {
    background: linear-gradient(90deg, #fbbf24, #f59e0b);
    box-shadow: 0 0 10px rgba(251, 191, 36, 0.3);
}

.ram-fill.high {
    background: linear-gradient(90deg, #fb923c, #f97316);
    box-shadow: 0 0 10px rgba(251, 146, 60, 0.3);
}

.ram-fill.critical {
    background: linear-gradient(90deg, #f87171, #ef4444);
    box-shadow: 0 0 10px rgba(248, 113, 113, 0.3);
    animation: pulse-ram 1s ease-in-out infinite;
}

@keyframes pulse-ram {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.6; }
}
</style>