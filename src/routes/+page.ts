import { invoke } from "@tauri-apps/api"
import type { Apartment } from "~/models/Apartment"
import type { PageLoad } from "./$types"

export const load = (async () => {
    return {
      apartments: await invoke<Apartment[]>("get_apartments")
    };
}) satisfies PageLoad
