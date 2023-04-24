import z from "zod"

export const GuestSchema = z.object({
    id: z.number().min(1),
    first_name: z.string().min(1, { message: "Il nome è obbligatorio" }),
    last_name: z.string().min(1, { message: "Il cognome è obbligatorio" }),
    birth_date: z.string().pipe(z.coerce.date())
})
export type Guest = z.infer<typeof GuestSchema>

export const NewGuestSchema = GuestSchema.omit({ id: true })
export type NewGuest = z.infer<typeof NewGuestSchema>
