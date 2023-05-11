import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api/tauri"
import { z } from "zod"
import { HouseWithApartmentsSchema } from "~/models/house"
import type { PageLoad } from "./$types"

export const load = (async () => {
  await invoke("get_apartments").then(d => console.log(d))

  const housesWithApartmentsParser = z.array(HouseWithApartmentsSchema)
  const housesWithApartmentsResult = housesWithApartmentsParser.safeParse(await invoke("get_apartments"))

  if (!housesWithApartmentsResult.success) {
    throw error(404, `${housesWithApartmentsResult.error.message}`)
  }


  return {
    title: "Pagina principale",
    description: "Vedi le prenotazioni, i prezzi e gli appartamenti per ogni casa. Inoltre, puoi aggiungere un ospite rapidamente",
    housesWithApartments: housesWithApartmentsResult.data
  }
}) satisfies PageLoad
