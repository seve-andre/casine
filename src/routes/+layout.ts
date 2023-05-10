import type { LayoutLoad } from "./$types"
import { getName } from "@tauri-apps/api/app"

export const ssr = false
export const prerender = true

export const load = (async () => {
  return {
    appName: await getName()
  }
}) satisfies LayoutLoad
