import { z } from "zod"
import { ApartmentSchema, GroupSchema, GuestSchema, RentSchema } from "~/models"

// json returns "null" if not found, so "nullable()"
export const ApartmentPageDataSchema = z.object({
  apartment: ApartmentSchema,
  rent: RentSchema.nullable(),
  guests: z.array(GuestSchema).nullable(),
  group: GroupSchema.nullable()
})

export type ApartmentPageData = z.infer<typeof ApartmentPageDataSchema>
