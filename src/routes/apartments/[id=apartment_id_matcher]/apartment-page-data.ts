import { z } from "zod"
import { ApartmentSchema, GroupSchema, GuestSchema, RentSchema } from "~/models"

export const ApartmentPageDataSchema = z.object({
    apartment: ApartmentSchema,
    rent: RentSchema,
    guests: z.array(GuestSchema),
    group: GroupSchema
})

export type ApartmentPageData = z.infer<typeof ApartmentPageDataSchema>
