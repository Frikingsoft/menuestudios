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

const lanzarApp = async (icono) => {
    try {
        console.log(`Lanzando: ${icono.nombre}`)
        
        // Usar comandos específicos según la aplicación
        switch(icono.nombre) {
            case 'firefox':
                await invoke('launch_firefox')
                break
            case 'terminal':
                // Usar Kitty con diferentes opciones
                // Opción 1: Terminal normal
                await invoke('launch_terminal')
                
                // Opción 2: Terminal en un directorio específico (descomentar para usar)
                // await invoke('launch_terminal_with_directory', { directory: '/home/usuario' })
                
                // Opción 3: Terminal ejecutando un comando (descomentar para usar)
                // await invoke('launch_terminal_with_command', { command: 'htop' })
                
                // Opción 4: Terminal en pantalla completa (descomentar para usar)
                // await invoke('launch_terminal_fullscreen')
                
                // Opción 5: Terminal con título (descomentar para usar)
                // await invoke('launch_terminal_with_title', { title: 'Mi Terminal' })
                break
            case 'vscode':
                await invoke('launch_app', { command: 'code' })
                break
            case 'archivos':
                await invoke('launch_app', { command: 'nautilus' })
                break
            case 'papelera':
                await invoke('launch_app', { command: 'gio trash' })
                break
            default:
                await invoke('launch_app', { command: icono.comando })
        }
        
        console.log(`✅ ${icono.nombre} lanzado correctamente`)
    } catch (error) {
        console.error(`❌ Error al lanzar ${icono.nombre}:`, error)
        alert(`No se pudo lanzar "${icono.nombre}". Verifica que esté instalado.`)
    }
}

// Función para verificar apps instaladas (opcional)
const verificarApps = async () => {
    try {
        const kittyInstalado = await invoke('check_kitty_installed')
        console.log(`Kitty instalado: ${kittyInstalado}`)
        
        if (kittyInstalado) {
            const version = await invoke('get_kitty_version')
            console.log(`Versión de Kitty: ${version}`)
        }
    } catch (error) {
        console.error('Error verificando apps:', error)
    }
}

// Ejecutar verificación al montar
verificarApps()
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
    }
    .imagen{
        width: 100%;
        height: 100%;
    }
</style>