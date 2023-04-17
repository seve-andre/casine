import z from "zod"
import type { PageLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api"
import { Guest } from "~/models/Guest"
import { Rent } from "~/models/Rent"

export const load = (async ({ params }) => {
  const guestsArrayParser = z.array(Guest)
  const guestsResult = guestsArrayParser.safeParse(await invoke("get_guests", { pHouseName: params.houseName, pApartmentNumber: +params.apartmentNumber}))

  const rentResult = Rent.safeParse(
    {
      id: 1,
      apartment_id: +params.apartmentNumber,
      group_id: 1,
      start_date: "2023-05-01",
      end_date: "2023-05-30",
    }
  )

  if (!guestsResult.success || !rentResult.success) {
    throw error(404, "Not found")
  }

  return {
    title: `Casa ${params.houseName} - Appartamento ${params.apartmentNumber}`,
    description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
    rentInfo: rentResult.data,
    guests: guestsResult.data
  }
}) satisfies PageLoad
