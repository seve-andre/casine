import z from "zod"
import { Constants } from "~/lib/utils/Constants"

export const Rent = z.object({
    id: z.number(),
    apartment_id: z.number().min(Constants.minApartmentId).max(Constants.maxApartmentId),
    group_id: z.number(),
    start_date: z.coerce.date(),
    end_date: z.coerce.date(),
})
