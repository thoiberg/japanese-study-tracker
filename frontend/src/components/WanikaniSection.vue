<template>
  <div v-if="waniKaniData">
    <p class="app-stats">Current Lessons: {{ waniKaniData.active_lesson_count }}</p>
    <p class="app-stats">Current Reviews: {{ waniKaniData.active_review_count }}</p>
    <UpdatedTimestamp :time-stamp="waniKaniData.data_updated_at" />
  </div>
  <div v-else-if="error">
    <p>{{ error.message }}</p>
  </div>
  <LoadingIndicator v-else />
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from 'vue'
import LoadingIndicator from './LoadingIndicator.vue'
import UpdatedTimestamp from './UpdatedTimestamp.vue'

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
</script>
