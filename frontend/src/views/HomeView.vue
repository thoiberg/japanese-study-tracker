<script setup lang="ts">
import WaniKani from '@/components/WaniKani.vue'
</script>

<template>
  <main class="grid-container">
    <div class="item-container wanikani">
      <WaniKani />
    </div>
    <div class="item-container">
      <p>Bunpro</p>
    </div>
    <div class="item-container">
      <p>Satori</p>
    </div>
    <div class="item-container">
      <p>Anki</p>
    </div>
  </main>
</template>

<style scoped>
.grid-container {
  height: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
}

.item-container {
  display: grid;
  grid-template-columns: subgrid;
  align-items: center;
  justify-items: center;
}

.wanikani {
  background-color: #df00a9;
  color: rgb(255, 255, 255);
}
</style>

<style>
.wanikani .loading-indicator {
  --accent-color: rgb(227, 219, 219);
}
</style>
