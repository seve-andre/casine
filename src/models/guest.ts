import z from "zod"
import { Constants } from "~/lib/utils/constants"

export const GuestSchema = z.object({
    id: z.number().min(1),
    first_name: z.string().min(1, { message: "Il nome è obbligatorio" }),
    last_name: z.string().min(1, { message: "Il cognome è obbligatorio" }),
    birth_date: z.string().pipe(z.coerce.date())
})
export type Guest = z.infer<typeof GuestSchema>

export const NewGuestSchema = GuestSchema
    .omit({ id: true, birth_date: true })
    .extend({
        // DB accepts DATE (NOT DATETIME), so we can't use `birth_date` of the
        // above GuestSchema
        birth_date: z.string().regex(Constants.dateRegex, { message: "La data di nascita non è corretta" })
    })
export type NewGuest = z.infer<typeof NewGuestSchema>
