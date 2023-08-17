import type { ParamMatcher } from "@sveltejs/kit"

import { Constants } from "$lib/utils/constants"

export const match = (param => {
  return Constants.apartmentRegex.test(param)
}) satisfies ParamMatcher
