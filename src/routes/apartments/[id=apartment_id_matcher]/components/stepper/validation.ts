import { NewGuestSchema, type NewGuest } from "$lib/models"

import { FirstStepSchema } from "./first/first-step"

export const useValidation = () => {
  const validatePeriodOfStay = (startDate: string, endDate: string) => {
    return FirstStepSchema.safeParse({ startDate, endDate })
  }
  const validateNewGuest = (newGuest: NewGuest) => {
    return NewGuestSchema.safeParse(newGuest)
  }

  return {
    validatePeriodOfStay,
    validateNewGuest
  }
}
