<script setup lang="ts">
import Bunpro from '@/components/BunproSection.vue'
import WaniKani from '@/components/WanikaniSection.vue'
import Satori from '@/components/SatoriSection.vue'
</script>

<template>
  <main class="grid-container">
    <div class="item-container wanikani">
      <WaniKani />
    </div>
    <div class="item-container bunpro">
      <Bunpro />
    </div>
    <div class="item-container satori">
      <Satori />
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

.bunpro {
  background-color: rgb(239, 239, 239);
  color: rgb(49, 117, 227);
}

.satori {
  background-color: rgb(255, 147, 0);
  color: rgb(255, 255, 255);
}
</style>

<style>
.wanikani {
  --accent-color: rgb(227, 219, 219);
}

.bunpro {
  --accent-color: rgb(49, 117, 227);
}

.satori {
  --accent-color: rgb(255, 255, 255);
}
</style>
