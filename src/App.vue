<script setup></script>

<template>
  <div class="menu1">
  </div>
    <nav class="menu2">
        <div class="boton-menu">
            <button>
            
            </button>
           
        </div>
        <div class="iconos">

        </div>
        <div class="fecha-hora">

        </div>
        <div class="paneles">

        </div>
    </nav>
</template>

<style>
  .menu1{
    position: absolute;
    bottom: 0;
    width: 100%;
    height: 100vh;
    background-color:rgb(182, 179, 25);
    overflow: hidden;
    
  }
  .menu2{
    display: grid;
    grid-template-columns: repeat(12, 1fr);
    gap: 8px;
    width: 95%;
    overflow: hidden;
    width:100%;
    left:0;
    height:48px;
    position:absolute;
    bottom:0;
    border: 2px solid rgba(73, 5, 70, 0.7);
    border-radius: 10px;
    background:
        radial-gradient(circle at center, rgba(61, 7, 68, 0.37), transparent 70%),
        linear-gradient(#02061a, #32033f);
      
    box-shadow:
         1px 2px 5px rgba(82, 4, 112, 0.692),
         2px 3px 6px rgb(77, 4, 77);
    
  }
.boton-menu{
    grid-column: 1 / 2 ; 
    display: flex;
    align-items: center;
    
}
.boton-menu button{
     width: 50px;
    height: 50px;
    background-image: url("/logo.png");
    background-size: 48px 48px;
    background-repeat: no-repeat;
     background-position: center;
    background-color: transparent;
    border: none;
    border-radius: 50%;
}
.boton-menu button:hover{
    border: 2px solid rgba(130, 40, 214, 0.699);
}

.iconos{
        grid-column: 2 / 6 ; 
        display: flex;
        align-items: center;
        background-color: white;
}
.fecha-hora{
        grid-column: 6 / 8 ; 
        display: flex;
        align-items: center;
        background-color: white;
}
.paneles{
    grid-column: 8 / 12 ; 
    display: flex;
    align-items: center;
    background-color: white;
}

</style>