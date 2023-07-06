<template>
  <p class="fetched-stats">Data Fetched at: {{ formatDateTime(props.timeStamp) }}</p>
</template>

<script setup lang="ts">
const props = defineProps<{
  timeStamp: string
}>()

function formatDateTime(timeStamp: string) {
  const date = new Date(timeStamp)

  return Intl.DateTimeFormat('en-AU', {
    dateStyle: 'short',
    timeStyle: 'short',
    timeZone: 'Japan'
  }).format(date)
}
</script>

<style scoped>
.fetched-stats {
  font-size: 1rem;
}
</style>
