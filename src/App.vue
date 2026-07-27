<template>
  <nav class="menu2">
    <div class="boton-menu">
      <button class="boton-inicio"></button>        
    </div>
    <div class="iconos">
      <Barra/>            
    </div>
    <div class="reloj-container">
      <div class="reloj-glow"></div>
      <div class="reloj">
        {{ horas }}<span class="separador">:</span>{{ minutos }}
      </div>
    </div>
    <div class="paneles"></div>
  </nav>
</template>

<script setup>
import Barra from "./componentes/Barra.vue"
import { ref, onMounted, onUnmounted } from 'vue'

const horas = ref('00')
const minutos = ref('00')
let intervalId = null

function actualizarHora() {
  const ahora = new Date()
  horas.value = String(ahora.getHours()).padStart(2, '0')
  minutos.value = String(ahora.getMinutes()).padStart(2, '0')
}

onMounted(() => {
  actualizarHora()
  intervalId = setInterval(actualizarHora, 1000)
})

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId)
})
</script>

<style>
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  .menu2 {
    display: grid;
    grid-template-columns: repeat(12, 1fr);
    gap: 8px;
    overflow: hidden;
    width: 100%;
    left: 0;
    height: 100%;
    position: absolute;
    bottom: 0;
  }

  .boton-menu {
    grid-column: 1 / 2; 
    display: flex;
    margin-left: 10px;
  }

  .boton-inicio {
    margin-top: 2px;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background-color: transparent;
    border: none;
    background-image: url("/logo.png");
    background-size: 100% 100%;
  }

  .boton-inicio:hover {
    border: 2px solid rgba(130, 40, 214, 0.699);
  }

  .iconos {
    grid-column: 2 / 6; 
    display: flex;
  }

  /* ===== RELOJ DIGITAL CYBERPUNK ===== */
  .reloj-container {
    grid-column: 6 / 8;
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    height: 48px;
  }

  .reloj-glow {
    position: absolute;
    width: 150px;
    height: 48px;
    border-radius: 12px;
    background: rgba(138, 43, 226, 0.15);
    filter: blur(20px);
    animation: pulse-glow 2s ease-in-out infinite;
  }

  .reloj {
    position: relative;
    z-index: 1;
    font-family: 'Courier New', 'Consolas', 'Monaco', monospace;
    font-size: 30px;
    font-weight: 900;
    color: #c084fc;
    letter-spacing: 6px;
    padding: 6px 20px;
    
    text-shadow: 
      0 0 5px #a855f7,
      0 0 10px #9333ea,
      0 0 20px #7e22ce,
      0 0 40px rgba(126, 34, 206, 0.3);
    
    border: 2px solid rgba(130, 40, 214, 0.4);
    border-radius: 10%/80%;
    
    box-shadow: 
      0 0 20px rgba(130, 40, 214, 0.3),
      0 0 40px rgba(130, 40, 214, 0.15),
      0 0 60px rgba(130, 40, 214, 0.08),
      inset 0 2px 4px rgba(255, 255, 255, 0.05),
      inset 0 -2px 4px rgba(0, 0, 0, 0.3),
      inset 0 0 0 1px rgba(168, 85, 247, 0.15),
      0 4px 6px rgba(0, 0, 0, 0.4),
      0 1px 3px rgba(0, 0, 0, 0.3);
    
    background: rgba(0, 0, 0, 0.815);
    backdrop-filter: blur(8px);
    transition: all 0.3s ease;
  }

  .reloj:hover {
    border-color: rgba(168, 85, 247, 0.9);
    box-shadow: 
      0 0 30px rgba(130, 40, 214, 0.4),
      0 0 60px rgba(130, 40, 214, 0.2),
      0 0 80px rgba(130, 40, 214, 0.1),
      inset 0 2px 4px rgba(255, 255, 255, 0.08),
      inset 0 -2px 4px rgba(0, 0, 0, 0.3),
      inset 0 0 0 1px rgba(168, 85, 247, 0.25),
      0 4px 6px rgba(0, 0, 0, 0.4),
      0 1px 3px rgba(0, 0, 0, 0.3);
  }

  .reloj::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border-radius: 10px;
    background: repeating-linear-gradient(
      0deg,
      transparent,
      transparent 2px,
      rgba(0, 0, 0, 0.08) 2px,
      rgba(0, 0, 0, 0.08) 4px
    );
    pointer-events: none;
    z-index: 2;
  }

  .reloj::after {
    content: '';
    position: absolute;
    top: -1px;
    left: -1px;
    right: -1px;
    bottom: -1px;
    border-radius: 11px;
    background: linear-gradient(
      135deg,
      rgba(168, 85, 247, 0.3) 0%,
      transparent 30%,
      transparent 70%,
      rgba(168, 85, 247, 0.15) 100%
    );
    pointer-events: none;
    z-index: 0;
    mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    mask-composite: exclude;
    -webkit-mask-composite: xor;
    padding: 1px;
  }

  .separador {
    animation: parpadear 1s step-end infinite;
    color: #a855f7;
  }

  @keyframes parpadear {
    0%, 100% { opacity: 1; }
    50% { opacity: 0; }
  }

  @keyframes pulse-glow {
    0%, 100% { 
      opacity: 0.6; 
      transform: scale(1);
    }
    50% { 
      opacity: 1; 
      transform: scale(1.05);
    }
  }

  .paneles {
    grid-column: 8 / 12; 
    display: flex;
    align-items: center;
  }
</style>