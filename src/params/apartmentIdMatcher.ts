import type { ParamMatcher } from "@sveltejs/kit"
import { Constants } from "~/lib/utils/Constants"

export const match = ((param) => {
  return Constants.apartmentRegex.test(param)
}) satisfies ParamMatcher
