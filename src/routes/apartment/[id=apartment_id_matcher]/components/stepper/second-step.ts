import { z } from "zod"
import { NewGuestSchema } from "~/models"

export const SecondStepSchema = z.object({
    guest: NewGuestSchema
})
