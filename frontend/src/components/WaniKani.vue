<template>
  <div v-if="waniKaniData">
    <div>Current Lessons: {{ waniKaniData.active_lesson_count }}</div>
    <div>Current Reviews: {{ waniKaniData.active_review_count }}</div>
    <div>Data Fetched at: {{ new Date(waniKaniData.data_updated_at) }}</div>
  </div>
  <div v-else-if="error">
    <p>{{ error.message }}</p>
  </div>
  <LoadingIndicator v-else class="loading" />
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from 'vue'
import LoadingIndicator from './LoadingIndicator.vue'

let waniKaniData: Ref<WaniKaniResponse | null> = ref(null)
let error: Ref<BackendError | null> = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('/api/wanikani')

    if (response.ok) {
      waniKaniData.value = await response.json()
    } else {
      error.value = await response.json()
    }
  } catch (e) {
    error.value = { message: 'Unexpected error occurred.' }
  }
})

type WaniKaniResponse = {
  data_updated_at: string
  active_lesson_count: number
  active_review_count: number
}

type BackendError = {
  message: string
}
</script>

<style scoped>
.loading {
  width: 33%;
}
</style>
