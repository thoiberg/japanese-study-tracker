import { z } from 'zod'

export type BackendError = z.infer<typeof ApiErrorResponse>

export const ApiErrorResponse = z.object({
  message: z.string()
})
