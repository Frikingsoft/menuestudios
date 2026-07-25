<template>
    <button v-for="(icono,item) in iconos" :key="item" class="botones" @click="lanzarApp(icono)"> 
        <img :src="icono.icono" class="imagen">
    </button>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core';

const iconos = ref([
    {nombre: "firefox", comando: "firefox", icono: "firefox.png"},
    {nombre: "terminal", comando: "kitty", icono: "terminal.png"},
    {nombre: "vscode", comando: "code", icono: "vscode.png"},
    {nombre: "archivos", comando: "nautilus", icono: "archivo.png"},
    {nombre: "papelera", comando: "gio trash", icono: "papelera.png"}
])

const lanzarApp = (icono) => {
    console.log(`Lanzando: ${icono.nombre}`)
    
    switch(icono.comando) {  // ← Usando icono.comando
        case 'firefox':
            invoke('launch_firefox')
                .then(() => console.log(`✅ ${icono.nombre} lanzado`))
                .catch((error) => {
                    console.error(`❌ Error:`, error)
                    alert(`No se pudo lanzar "${icono.nombre}"`)
                })
            break
        case 'kitty':
            invoke('launch_terminal')
                .then(() => console.log(`✅ ${icono.nombre} lanzado`))
                .catch((error) => {
                    console.error(`❌ Error:`, error)
                    alert(`No se pudo lanzar "${icono.nombre}"`)
                })
            break
        case "code":
            invoke("launch_vscode")
            .then(()=>console.log(`✅ ${icono.nombre} lanzado`))
            .catch((error) => {
                    console.error(`❌ Error:`, error)
                    alert(`No se pudo lanzar "${icono.nombre}"`)
                })
            break    
        default:
            console.log(`⏳ ${icono.nombre} - Pendiente de implementar`)
    }
}
</script>

<style scoped>
    .botones{
        width: 40px;
        height: 40px;
        margin-top: 4px;
        margin-left: 20px;
        background-color: transparent;
        border: none;
        cursor: pointer;
        transition: all 0.2s ease;

    }
    .botones:hover {
        transform: scale(1.1);
        animation: animar .5s infinite linear;
         
    }
 
    .imagen{
        width: 100%;
        height: 100%;
    }
    @keyframes animar {
        0%{
            margin-top: 0;
        }
        30%{
            margin-top: -5px;
        }
        80%{
            margin-top: 5px;
        }
        100%{
            margin-top: 0;
        }        
    }
</style>