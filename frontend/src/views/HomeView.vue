<script setup lang="ts">
import WaniKani from '@/components/WaniKani.vue'
</script>

<template>
  <main class="grid-container">
    <div class="item-container">
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
}

.item-container {
  display: grid;
  grid-template-columns: subgrid;
  align-items: center;
  justify-items: center;
}
</style>
