import z from "zod"

export const Guest = z.object({
    id: z.number(),
    first_name: z.string(),
    last_name: z.string(),
    birth_date: z.coerce.date()
})
