<template>
  <div class="app-stats" v-if="satoriData">
    <DailyGoalIndicator v-if="satoriData.daily_study_goal_met" />
    <p>Current Reviews: {{ satoriData.active_review_count }}</p>
    <p>New Cards: {{ satoriData.new_card_count }}</p>
    <UpdatedTimestamp :time-stamp="satoriData.data_updated_at" />
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
import { type BackendError, ApiErrorResponse } from '@/utils/errorParsing'
import { z } from 'zod'
import DailyGoalIndicator from './DailyGoalIndicator.vue'

let satoriData: Ref<SatoriResponse | null> = ref(null)
let error: Ref<BackendError | null> = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('/api/satori')

    if (response.ok) {
      satoriData.value = SatoriResponseSchema.parse(await response.json())
    } else {
      error.value = ApiErrorResponse.parse(await response.json())
    }
  } catch (e) {
    error.value = { message: 'Unexpected error occurred.' }
  }
})

const SatoriResponseSchema = z.object({
  data_updated_at: z.string(),
  new_card_count: z.number(),
  active_review_count: z.number(),
  daily_study_goal_met: z.boolean()
})

export type SatoriResponse = z.infer<typeof SatoriResponseSchema>
</script>
