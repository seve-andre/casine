import { getName } from "@tauri-apps/api/app"

import type { LayoutLoad } from "./$types"

export const ssr = false
export const prerender = true

export const load = (async () => {
  return {
    appName: await getName()
  }
}) satisfies LayoutLoad
