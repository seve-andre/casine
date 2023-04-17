import z from "zod"

export const Rent = z.object({
    id: z.number(),
    apartment_id: z.number().lte(12),
    group_id: z.number(),
    start_date: z.date(),
    end_date: z.date(),
})
