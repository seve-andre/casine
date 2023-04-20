import type { PageLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api"
import { z } from "zod"
import { ApartmentSchema } from "~/models/apartment"
import { GuestSchema } from "~/models/guest"
import { RentSchema } from "~/models/rent"

export const load = (async ({ params }) => {
  const apartmentId = +params.id
  const apartmentResult = ApartmentSchema.safeParse(
    await invoke("get_apartment_by_id", { apartmentId })
  )
  if (!apartmentResult.success) {
    throw error(404, "Not Found")
  }

  const guestsArrayParser = z.array(GuestSchema)
  const guestsResult = guestsArrayParser.safeParse(
    await invoke("get_guests", { apartment: apartmentResult.data })
  )
  if (!guestsResult.success) {
    throw error(404, "Not Found")
  }

  const rentResult = RentSchema.safeParse(
    {
      id: 1,
      apartment_id: 1,
      group_id: 1,
      start_date: "2023-05-01",
      end_date: "2023-05-30",
    }
  )
  if (!rentResult.success) {
    throw error(404, "Not Found")
  }


  return {
    title: `Casa ${apartmentResult.data.house_name} - Appartamento ${apartmentResult.data.apartment_number}`,
    description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
    rent: rentResult.data,
    guests: guestsResult.data
  }
}) satisfies PageLoad
