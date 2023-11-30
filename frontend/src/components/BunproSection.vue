<template>
  <div class="app-stats" v-if="bunproData">
    <DailyGoalIndicator v-if="bunproData.daily_study_goal_met" />
    <p>Current Reviews: {{ bunproData.active_review_count }}</p>
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
import DailyGoalIndicator from './DailyGoalIndicator.vue'
import { type BackendError, ApiErrorResponse } from '@/utils/errorParsing'
import { z } from 'zod'

let bunproData: Ref<BunproResponse | null> = ref(null)
let error: Ref<BackendError | null> = ref(null)

onMounted(async () => {
  try {
    const response = await fetch('/api/bunpro')

    if (response.ok) {
      bunproData.value = BunproResponseSchema.parse(await response.json())
    } else {
      error.value = ApiErrorResponse.parse(await response.json())
    }
  } catch (e) {
    error.value = { message: 'Unexpected error occurred.' }
  }
})

const BunproResponseSchema = z.object({
  data_updated_at: z.string(),
  active_review_count: z.number(),
  daily_study_goal_met: z.boolean()
})

export type BunproResponse = z.infer<typeof BunproResponseSchema>
</script>
