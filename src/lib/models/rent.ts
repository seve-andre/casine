import z from "zod"

import { Constants } from "$lib/utils/constants"

export const RentSchema = z.object({
  id: z.number().min(1),
  apartment_id: z.number().min(Constants.minApartmentId).max(Constants.maxApartmentId),
  group_id: z.number().min(1),
  start_date: z.string().pipe(z.coerce.date()),
  end_date: z.string().pipe(z.coerce.date())
})

export type Rent = z.infer<typeof RentSchema>
