import type { ParamMatcher } from "@sveltejs/kit"

export const match = ((param) => {
  return /A|B/.test(param)
}) satisfies ParamMatcher
