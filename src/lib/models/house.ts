import { z } from "zod"

import { Constants } from "$lib/utils/constants"

import { ApartmentSchema } from "./apartment"

export const HouseSchema = z.object({
  house_name: z.string().regex(Constants.houseRegex),
  street_type: z.string(),
  street_name: z.string(),
  street_number: z.number()
})

export type House = z.infer<typeof HouseSchema>

export const HouseWithApartmentsSchema = z.object({
  house: HouseSchema,
  apartments: z.array(ApartmentSchema)
})

export type HouseWithApartments = z.infer<typeof HouseWithApartmentsSchema>
