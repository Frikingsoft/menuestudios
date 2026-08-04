<template>
  <div class="system-container">
    
    <Memoria />
    
    <Cpu />
    
    <Disco />
  
    <Sonido />
    <!--div class="separador-vertical"></div>
    <Pan
    talla />
    <div class="separador-vertical"></div>
    <Configuraciones /-->
  </div>
</template>

<script setup>
import Memoria from './barra2/Memoria.vue';
import Cpu from './barra2/CPU.vue';
import Disco from './barra2/Disco.vue';
import Sonido from './barra2/Sonido.vue';
//import Pantalla from './sistema/Pantalla.vue';
//import Configuraciones from './sistema/Configuraciones.vue';
</script>

<style scoped>
.system-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 4px 12px;
 
  
  border-radius: 8px;
  transition: all 0.3s ease;
  height: 40px;
}



.separador-vertical {
  width: 1px;
  height: 25px;
  background: rgba(131, 100, 255, 0.2);
  flex-shrink: 0;
}

@media (max-width: 1024px) {
  .system-container {
    gap: 8px;
    padding: 4px 8px;
  }
}

@media (max-width: 768px) {
  .system-container {
    padding: 4px 6px;
    height: 36px;
    gap: 6px;
  }
  
  .separador-vertical {
    height: 20px;
  }
}

@media (max-width: 480px) {
  .system-container {
    padding: 4px 4px;
    gap: 4px;
    height: 34px;
  }
  
  .separador-vertical {
    display: none;
  }
}
</style>