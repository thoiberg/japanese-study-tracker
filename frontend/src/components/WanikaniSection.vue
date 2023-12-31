<template>
  <div class="app-stats" v-if="wanikaniData">
    <DailyGoalIndicator v-if="wanikaniData.daily_study_goal_met" />
    <p>Current Reviews: {{ wanikaniData.active_review_count }}</p>
    <p>New Lessons: {{ wanikaniData.active_lesson_count }}</p>
    <UpdatedTimestamp :time-stamp="wanikaniData.data_updated_at" />
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

let wanikaniData: Ref<WanikaniResponse | null> = ref(null)
let error: Ref<BackendError | null> = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('/api/wanikani')

    if (response.ok) {
      wanikaniData.value = WanikaniResponseSchema.parse(await response.json())
    } else {
      error.value = ApiErrorResponse.parse(await response.json())
    }
  } catch (e) {
    error.value = { message: 'Unexpected error occurred.' }
  }
})

const WanikaniResponseSchema = z.object({
  data_updated_at: z.string(),
  active_lesson_count: z.number(),
  active_review_count: z.number(),
  daily_study_goal_met: z.boolean()
})

export type WanikaniResponse = z.infer<typeof WanikaniResponseSchema>
</script>
