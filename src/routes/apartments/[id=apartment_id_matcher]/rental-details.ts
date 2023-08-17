import { z } from "zod"

import { ApartmentSchema, GroupSchema, GuestSchema, RentSchema } from "$lib/models"

// json returns "null" if not found, so "nullable()"
export const RentalDetailsSchema = z.object({
  apartment: ApartmentSchema,
  rent: RentSchema.nullable(),
  guests: z.array(GuestSchema).nullable(),
  group: GroupSchema.nullable()
})

export type RentalDetails = z.infer<typeof RentalDetailsSchema>
