<template>
  <div v-if="ankiData">
    <p class="app-stats">Current Reviews: {{ ankiData.active_review_count }}</p>
    <p class="app-stats">New Cards: {{ ankiData.new_card_count }}</p>
    <UpdatedTimestamp :time-stamp="ankiData.data_updated_at" />
  </div>
  <div v-else-if="error">
    <p>{{ error.message }}</p>
  </div>
  <LoadingIndicator v-else />
</template>

<script setup lang="ts">
import { ApiErrorResponse, type BackendError } from '@/utils/errorParsing'
import LoadingIndicator from './LoadingIndicator.vue'
import UpdatedTimestamp from './UpdatedTimestamp.vue'
import { onMounted, ref, type Ref } from 'vue'
import { z } from 'zod'

const ankiData: Ref<AnkiResponse | null> = ref(null)
const error: Ref<BackendError | null> = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('/api/anki')

    if (response.ok) {
      ankiData.value = AnkiResponseSchema.parse(await response.json())
    } else {
      error.value = ApiErrorResponse.parse(await response.json())
    }
  } catch (e) {
    error.value = { message: 'Unexpected error occurred.' }
  }
})

const AnkiResponseSchema = z.object({
  active_review_count: z.number(),
  new_card_count: z.number(),
  data_updated_at: z.string()
})

type AnkiResponse = z.infer<typeof AnkiResponseSchema>
</script>
