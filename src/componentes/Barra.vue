<template>
    <button v-for="(icono,item) in iconos" :key="item" class="botones"> 
        <img :src="icono.icono" class="imagen">
    </button>
</template>
<script setup>
import { ref } from "vue";

    const iconos=ref([
        {nombre:"firefox",comando:"", icono:"firefox.png"},
        {nombre:"terminal",comando:"", icono:"terminal.png"},
        {nombre:"vscode",comando:"", icono:"vscode.png"},
        {nombre:"archivos",comando:"", icono:"archivo.png"},
        {nombre:"papelera",comando:"", icono:"papelera.png"}
    ])
</script>

<style scoped>
    .botones{
        width: 40px;
        height: 40px;
        margin-top: 4px;
        margin-left: 20px;
        background-color: transparent;
        border: none;
    }
    .imagen{
        width: 100%;
        height: 100%;
        
    }
</style>