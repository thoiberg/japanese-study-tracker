<script setup lang="ts">
import Bunpro from '@/components/BunproSection.vue'
import WaniKani from '@/components/WanikaniSection.vue'
import Satori from '@/components/SatoriSection.vue'
import Anki from '@/components/AnkiSection.vue'
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
    <div class="item-container anki">
      <Anki />
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
  --accent-color: hsl(0, 0%, 100%);
  --main-color: hsl(315, 100%, 44%);
}

.bunpro {
  --accent-color: hsl(217, 76%, 54%);
  --main-color: hsl(0, 0%, 94%);
}

.satori {
  --accent-color: hsl(0, 0%, 100%);
  --main-color: hsl(35, 100%, 50%);
}

.anki {
  --accent-color: hsl(205, 95%, 64%);
  --main-color: hsl(0, 0%, 93%);
}
</style>
