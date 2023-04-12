import { invoke } from "@tauri-apps/api"
import type { Apartment } from "~/models/Apartment"
import type { PageLoad } from "./$types"

export const load = (async () => {
    return {
      title: "Pagina principale",
      description: "Vedi le prenotazioni, i prezzi e gli appartamenti per ogni casa. Inoltre, puoi aggiungere un ospite rapidamente",
      apartments: await invoke<Apartment[]>("get_apartments")
    }
}) satisfies PageLoad
