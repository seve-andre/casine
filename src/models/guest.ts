import z from "zod"

export const GuestSchema = z.object({
    id: z.number(),
    first_name: z.string(),
    last_name: z.string(),
    birth_date: z.coerce.date()
})
export type Guest = z.infer<typeof GuestSchema>

export const NewGuestSchema = GuestSchema.omit({ id: true})
export type NewGuest = z.infer<typeof NewGuestSchema>
