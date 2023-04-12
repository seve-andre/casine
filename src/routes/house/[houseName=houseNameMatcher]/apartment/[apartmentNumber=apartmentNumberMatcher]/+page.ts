import type { PageLoad } from "./$types"

export const load = (async ({ params }) => {
    return {
      title: `Casa ${params.houseName} - Appartamento ${params.apartmentNumber}`,
      description: "Vedi, aggiungi o rimuovi gli ospiti dall'appartamento"
    }
}) satisfies PageLoad
