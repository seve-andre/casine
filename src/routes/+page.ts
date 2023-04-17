import { error } from "@sveltejs/kit"
import { invoke } from "@tauri-apps/api"
import { z } from "zod"
import { ApartmentSchema } from "~/models/apartment"
import type { PageLoad } from "./$types"

export const load = (async () => {
  const apartmentArrayParser = z.array(ApartmentSchema)
  const apartmentsResult = apartmentArrayParser.safeParse(await invoke("get_apartments"))

  if (!apartmentsResult.success) {
    throw error(404, "Not found")
  }

  return {
    title: "Pagina principale",
    description: "Vedi le prenotazioni, i prezzi e gli appartamenti per ogni casa. Inoltre, puoi aggiungere un ospite rapidamente",
    apartments: apartmentsResult.data
  }
}) satisfies PageLoad
