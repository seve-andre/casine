import { z } from "zod"

export const FirstStepSchema = z.object({
    startDate: z.string().pipe(z.coerce.date({
        errorMap: (issue, ctx) => {
            if (issue.code === z.ZodIssueCode.invalid_date) {
                return { message: "La data di arrivo è invalida"}
            }

            return { message: ctx.defaultError }
        }
    })),

    endDate: z.string().pipe(z.coerce.date({
        errorMap: (issue, ctx) => {
            if (issue.code === z.ZodIssueCode.invalid_date) {
                return { message: "La data di partenza è invalida"}
            }

            return { message: ctx.defaultError }
        }
    }))
})
.refine(
    data => data.endDate > data.startDate, {
        message: "La data di arrivo deve essere prima della data di partenza",
        path: ["startDate"]
    }
)

export type FirstStepErrors = {
    onStartDate?: string
    onEndDate?: string
}

export const firstStepErrorsDefaults: Pick<FirstStepErrors, "onStartDate" | "onEndDate"> = {
    onStartDate: undefined,
    onEndDate: undefined
}
