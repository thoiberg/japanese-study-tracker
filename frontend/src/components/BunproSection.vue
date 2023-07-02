<template>
  <div v-if="bunproData">
    <p class="app-stats">Current Reviews: {{ bunproData.active_review_count }}</p>
    <UpdatedTimestamp :time-stamp="bunproData.data_updated_at" />
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

let bunproData: Ref<BunproResponse | null> = ref(null)
let error: Ref<BackendError | null> = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('/api/bunpro')

    if (response.ok) {
      bunproData.value = await response.json()
    } else {
      error.value = await response.json()
    }
  } catch (e) {
    error.value = { message: 'Unexpected error occurred.' }
  }
})

type BunproResponse = {
  data_updated_at: string
  active_review_count: number
}
</script>