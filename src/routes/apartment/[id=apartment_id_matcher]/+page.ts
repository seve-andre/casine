import type { PageLoad } from "./$types"
import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api"
import { ApartmentPageDataSchema } from "./ApartmentPageData"

export const load = (async ({ params }) => {
  const apartmentId = +params.id

  const pageDataResult = ApartmentPageDataSchema.safeParse({
    apartment: await invoke("get_apartment_by_id", { apartmentId }),
    rent: {
      id: 1,
      apartment_id: 1,
      group_id: 1,
      start_date: "2023-05-01",
      end_date: "2023-05-30",
    },
    guests: await invoke("get_guests_in_apartment", { apartmentId }),
    group: {
      id: 1,
      nickname: "Severi"
    }
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
