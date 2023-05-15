import { z } from "zod"

export const FirstStepSchema = z.object({
    startDate: z.string().pipe(z.coerce.date()),
    endDate: z.string().pipe(z.coerce.date()),
})
.refine(
    data => data.endDate > data.startDate,
    "La data di arrivo deve essere prima della data di partenza"
)

export const FirstStep = {
    number: 1,
    name: "1 - Scegli periodo",
    // validator: validateDates("", ""),
}
