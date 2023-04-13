import { invoke } from "@tauri-apps/api"
import type { PageLoad } from "./$types"
import type { Guest } from "~/models/Guest"

export const load = (async ({ params }) => {
    return {
      title: `Casa ${params.houseName} - Appartamento ${params.apartmentNumber}`,
      description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
      guests: await invoke<Guest[]>("get_guests", { pHouseName: params.houseName, pApartmentNumber: +params.apartmentNumber})
    }
}) satisfies PageLoad
