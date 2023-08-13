<template>
  <p>
    {{ props.cardType }}: {{ props.cappedCount }}
    <span class="super" v-if="props.totalCount > props.cappedCount">
      (+ {{ props.totalCount - props.cappedCount }})
    </span>
  </p>
</template>

<script setup lang="ts">
const props = defineProps<{
  cardType: string
  cappedCount: number
  totalCount: number
}>()
</script>

<style>
.super {
  font-size: 1rem;
  vertical-align: text-top;
}
</style>
