<template>
  <div v-if="waniKaniData">
    <div>{{ waniKaniData.active_lesson_count }}</div>
    <div>{{ waniKaniData.active_review_count }}</div>
    <div>{{ new Date(waniKaniData.data_updated_at) }}</div>
  </div>
  <div v-else>Loading...</div>
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from 'vue'

let waniKaniData: Ref<WaniKaniResponse | null> = ref(null)

onMounted(async () => {
  const response = await fetch('/api/wanikani')
  waniKaniData.value = await response.json()
})

type WaniKaniResponse = {
  data_updated_at: string
  active_lesson_count: number
  active_review_count: number
}
</script>
