import type { ParamMatcher } from "@sveltejs/kit"
import { Constants } from "~/lib/utils/Constants"

export const match = ((param) => {
  return Constants.houseRegex.test(param)
}) satisfies ParamMatcher
