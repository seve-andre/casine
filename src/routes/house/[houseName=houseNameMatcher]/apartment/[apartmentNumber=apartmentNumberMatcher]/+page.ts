import { invoke } from "@tauri-apps/api"
import type { PageLoad } from "./$types"
import type { Guest } from "~/models/Guest"
import type { Rent } from "~/models/Rent"

export const load = (async ({ params }) => {
    return {
      title: `Casa ${params.houseName} - Appartamento ${params.apartmentNumber}`,
      description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
      rentInfo: {
        id: 1,
        apartment_id: params.apartmentNumber,
        group_id: 1,
        start_date: new Date("2023-05-01"),
        end_date: new Date("2023-05-30"),
      } as unknown as Rent,
      guests: await invoke<Guest[]>("get_guests", { pHouseName: params.houseName, pApartmentNumber: +params.apartmentNumber})
    }
}) satisfies PageLoad
