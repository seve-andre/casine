import z from "zod"
import { Constants } from "~/lib/utils/constants"

export const RentSchema = z.object({
    id: z.number(),
    apartment_id: z.number().min(Constants.minApartmentId).max(Constants.maxApartmentId),
    group_id: z.number(),
    start_date: z.coerce.date(),
    end_date: z.coerce.date(),
})

export type Rent = z.infer<typeof RentSchema>

