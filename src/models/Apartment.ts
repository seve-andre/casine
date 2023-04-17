import { z } from "zod"

export const Apartment = z.object({
    id: z.number(),
    house_name: z.string().regex(/A|B/g),
    apartment_number: z.number().lte(6)
})
