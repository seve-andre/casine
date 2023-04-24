import { z } from "zod"
import { GuestSchema } from "~/models"

export const SecondStepSchema = z.object({
    guest: GuestSchema
})
