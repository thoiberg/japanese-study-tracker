<template>
  <div class="app-stats" v-if="ankiData">
    <DailyGoalIndicator v-if="ankiData.daily_study_goal_met" />
    <CappedCount
      card-type="Current Reviews"
      :capped-count="ankiData.active_review_count"
      :total-count="ankiData.total_active_review_count"
    ></CappedCount>
    <CappedCount
      card-type="New Cards"
      :capped-count="ankiData.new_card_count"
      :total-count="ankiData.total_new_card_count"
    ></CappedCount>
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
import DailyGoalIndicator from './DailyGoalIndicator.vue'
import { onMounted, ref, type Ref } from 'vue'
import { z } from 'zod'
import CappedCount from './CappedCount.vue'

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
  total_active_review_count: z.number(),
  new_card_count: z.number(),
  data_updated_at: z.string(),
  total_new_card_count: z.number(),
  daily_study_goal_met: z.boolean()
})

export type AnkiResponse = z.infer<typeof AnkiResponseSchema>
</script>

<style>
.super {
  font-size: 1rem;
  vertical-align: text-top;
}
</style>
