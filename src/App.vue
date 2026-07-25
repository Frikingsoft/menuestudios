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
        {{ hora }}
      </div>
    </div>
    <div class="paneles"></div>
  </nav>
</template>

<script setup>
import Barra from "./componentes/Barra.vue"
import { ref, onMounted, onUnmounted } from 'vue'

const hora = ref('')
let intervalId = null

function actualizarHora() {
  const ahora = new Date()
  const horas = String(ahora.getHours()).padStart(2, '0')
  const minutos = String(ahora.getMinutes()).padStart(2, '0')
  hora.value = `${horas}:${minutos}`
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
    background-color:  rgba(43, 7, 77, 0.699);
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

  /* ===== RELOJ ESTILO IMAGEN ===== */
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
    width: 120px;
    height: 40px;
    border-radius: 20px;
    background: rgba(138, 43, 226, 0.15);
    filter: blur(12px);
    animation: pulse-glow 2s ease-in-out infinite;
  }

  .reloj {
    position: relative;
    z-index: 1;
    font-family: 'Courier New', 'Consolas', 'Monaco', monospace;
    font-size: 28px;
    font-weight: 700;
    color: #c084fc;
    letter-spacing: 4px;
    text-shadow: 
      0 0 5px #a855f7,
      0 0 10px #9333ea,
      0 0 20px #7e22ce,
      0 0 40px rgba(126, 34, 206, 0.5);
    padding: 0 12px;
  }

  @keyframes pulse-glow {
    0%, 100% { opacity: 0.6; transform: scale(1); }
    50% { opacity: 1; transform: scale(1.05); }
  }

  /* Separador parpadeante (:) */
  .reloj::after {
    content: '';
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 2px;
    height: 20px;
    background: transparent;
  }

  .paneles {
    grid-column: 8 / 12; 
    display: flex;
    align-items: center;
    
  }
</style>