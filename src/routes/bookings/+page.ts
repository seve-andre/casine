import type { PageLoad } from "./$types"

export const load = (async () => {
  // get bookings

  return {
    title: "Prenotazioni",
    description: "Vedi le prenotazioni relative agli appartamenti"
  }
}) satisfies PageLoad
