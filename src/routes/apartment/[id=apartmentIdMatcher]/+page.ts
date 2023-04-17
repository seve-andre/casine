import type { PageLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api"
import { Apartment } from "~/models/Apartment"
import { z } from "zod"
import { Guest } from "~/models/Guest"
import { Rent } from "~/models/Rent"

export const load = (async ({ params }) => {
  const apartmentId = +params.id
  const apartmentResult = Apartment.safeParse(await invoke("get_apartment_by_id", { apartmentId }))
  if (!apartmentResult.success) {
    throw error(404, "Not found")
  }

  const guestsArrayParser = z.array(Guest)
  const guestsResult = guestsArrayParser.safeParse(
    await invoke(
      "get_guests",
      {
        houseName: apartmentResult.data.house_name,
        apartmentNumber: apartmentResult.data.apartment_number
      }
    )
  )
  if (!guestsResult.success) {
    throw error(404, "Not found")
  }

  const rentResult = Rent.safeParse(
    {
      id: 1,
      apartment_id: 1,
      group_id: 1,
      start_date: "2023-05-01",
      end_date: "2023-05-30",
    }
  )
  if (!rentResult.success) {
    throw error(404, "Not found")
  }


  return {
    title: `Casa ${apartmentResult.data.house_name} - Appartamento ${apartmentResult.data.apartment_number}`,
    description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
    rent: rentResult.data,
    guests: guestsResult.data
  }
}) satisfies PageLoad
