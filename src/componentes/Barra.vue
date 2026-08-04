<template>
    <button 
        v-for="(icono,item) in iconos" 
        :key="item" 
        class="botones" 
        @click="lanzarApp(icono)"
        @mouseenter="hoverItem = item"
        @mouseleave="hoverItem = null"
    > 
        <img 
            :src="icono.icono" 
            class="imagen"
            :class="{ animando: hoverItem === item }"
            :alt="icono.nombre"
        >
    </button>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core';


const hoverItem = ref(null)

const iconos = ref([
    {nombre: "firefox", comando: "firefox", icono: "firefox.png"},
    {nombre: "terminal", comando: "kitty", icono: "terminal.png"},
    {nombre: "vscode", comando: "code", icono: "vscode.png"},
    {nombre: "archivos", comando: "nautilus", icono: "archivo.png"},
    {nombre: "papelera", comando: "gio trash", icono: "papelera.png"},
    {nombre: "cerrar", comando: "cerrar", icono: "cerrar.png"},
    {nombre: "reiniciar", comando: "reiniciar", icono: "reiniciar.png"},
    {nombre: "apagar", comando: "apagar", icono: "apagar.png"}
])

const lanzarApp = (icono) => {
    console.log(`Lanzando: ${icono.nombre}`)
    
    switch(icono.comando) {
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

        case "cerrar":
            invoke('cerrar_sesion')
                .then(() => console.log(`✅ ${icono.nombre} ejecutado`))
                .catch((error) => {
                    console.error(`❌ Error:`, error)
                    alert(`No se pudo cerrar sesión`)
                })
            break

        case "reiniciar":
            invoke('reiniciar_sistema')
                .then(() => console.log(`✅ ${icono.nombre} ejecutado`))
                .catch((error) => {
                    console.error(`❌ Error:`, error)
                    alert(`No se pudo reiniciar el sistema`)
                })
            break

        case "apagar":
            invoke('apagar_sistema')
                .then(() => console.log(`✅ ${icono.nombre} ejecutado`))
                .catch((error) => {
                    console.error(`❌ Error:`, error)
                    alert(`No se pudo apagar el sistema`)
                })
            break
            
        default:
            console.log(`⏳ ${icono.nombre} - Pendiente de implementar`)
    }
}
</script>

<style scoped>
.botones {
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
}

.imagen {
    width: 100%;
    height: 100%;
    transition: transform 0.3s ease;
}

.imagen.animando {
    animation: animar 0.8s ease-in-out;
}

@keyframes animar {
    0% {
        transform: translateY(0) scale(1);
    }
    20% {
        transform: translateY(-8px) scale(1.05);
    }
    40% {
        transform: translateY(0) scale(1);
    }
    60% {
        transform: translateY(-4px) scale(1.02);
    }
    80% {
        transform: translateY(0) scale(1);
    }
    100% {
        transform: translateY(0) scale(1);
    }
}

/* Estilo para cerrar sesión (amarillo) */
.botones:nth-last-child(3) .imagen {
    filter: brightness(1.2);
}

.botones:nth-last-child(3):hover .imagen {
    filter: brightness(1.5) drop-shadow(0 0 8px rgba(255, 255, 0, 0.5));
}

/* Estilo para reiniciar (cian) */
.botones:nth-last-child(2) .imagen {
    filter: brightness(1.2);
}

.botones:nth-last-child(2):hover .imagen {
    filter: brightness(1.5) drop-shadow(0 0 8px rgba(0, 255, 255, 0.5));
}

/* Estilo para apagar (rojo) */
.botones:last-child .imagen {
    filter: brightness(1.2);
}

.botones:last-child:hover .imagen {
    filter: brightness(1.5) drop-shadow(0 0 8px rgba(255, 0, 0, 0.5));
}
</style>