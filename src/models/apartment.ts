import { z } from "zod"
import { Constants } from "~/lib/utils/constants"

export const ApartmentSchema = z.object({
    id: z.number(),
    house_name: z.string().regex(Constants.houseRegex),
    apartment_number: z.number().min(Constants.minApartmentNumber).max(Constants.maxApartmentNumber)
})

export type Apartment = z.infer<typeof ApartmentSchema>
