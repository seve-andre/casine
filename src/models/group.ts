import { z } from "zod"

export const GroupSchema = z.object({
    id: z.number(),
    nickname: z.string()
})

export type Group = z.infer<typeof GroupSchema>
