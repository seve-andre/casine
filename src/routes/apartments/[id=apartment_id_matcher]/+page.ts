import type { PageLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api/tauri"
import { ApartmentPageDataSchema } from "./apartment-page-data"

export const load = (async ({ params }) => {
  const apartmentId = +params.id

  const pageDataResult = ApartmentPageDataSchema.safeParse({
    apartment: await invoke("get_apartment_by_id", { apartmentId }),
    rent: await invoke("get_rent_in_apartment", { apartmentId }),
    guests: await invoke("get_guests_in_apartment", { apartmentId }),
    group: await invoke("get_group_in_apartment", { apartmentId })
  })

  if (!pageDataResult.success) {
    throw error(404, "Not Found")
  }

  const { apartment, rent, guests, group } = pageDataResult.data


  return {
    title: `Casa ${apartment.house_name} - Appartamento ${apartment.apartment_number}`,
    description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento",
    rent: rent,
    guests: guests,
    group: group
  }
}) satisfies PageLoad

export const prerender = false
