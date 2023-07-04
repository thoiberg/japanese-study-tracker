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
  min-height: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
}

@media screen and (max-width: 726px) {
  .grid-container {
    grid-template-columns: 1fr;
    grid-template-rows: 1fr 1fr 1fr 1fr;
  }
}

.item-container {
  display: grid;
  grid-template-columns: subgrid;
  align-items: center;
  justify-items: center;

  background-color: var(--main-color, --color-background);
  color: var(--accent-color, --color-text);
}
</style>

<style>
.wanikani {
  --accent-color: rgb(255, 255, 255);
  --main-color: #df00a9;
}

.bunpro {
  --accent-color: rgb(49, 117, 227);
  --main-color: rgb(239, 239, 239);
}

.satori {
  --accent-color: rgb(255, 255, 255);
  --main-color: rgb(255, 147, 0);
}
</style>
