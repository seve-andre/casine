import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api/tauri"

import type { PageLoad } from "./$types"
import { RentalDetailsSchema } from "./rental-details"

export const load = (async ({ params }) => {
  const apartmentId = +params.id

  const rentailDetailsResult = RentalDetailsSchema.safeParse(
    await invoke("get_rental_details", { apartmentId })
  )

  if (!rentailDetailsResult.success) {
    throw error(404, "Not Found")
  }

  const { apartment, rent, guests, group } = rentailDetailsResult.data

  return {
    title: `Casa ${apartment.house_name} - Appartamento ${apartment.apartment_number}`,
    description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
    rent: rent,
    guests: guests,
    group: group
  }
}) satisfies PageLoad

export const prerender = false
