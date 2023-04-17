import z from "zod"

export const GuestSchema = z.object({
    id: z.number(),
    first_name: z.string(),
    last_name: z.string(),
    birth_date: z.coerce.date()
})

export type Guest = z.infer<typeof GuestSchema>
