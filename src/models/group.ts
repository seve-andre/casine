import { z } from "zod"

export const GroupSchema = z.object({
    id: z.number().min(1),
    nickname: z.string().min(1)
})
export type Group = z.infer<typeof GroupSchema>

export const NewGroupSchema = GroupSchema.omit({ id: true })
export type NewGroup = z.infer<typeof NewGroupSchema>
