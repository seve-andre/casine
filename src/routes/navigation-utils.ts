import { goto } from "$app/navigation"

export const gotoHome = () => goto("/")
export const gotoApartmentInHouse = (apartmentNumber: number, houseName: string) => goto(`house/${houseName}/apartment/${apartmentNumber}`)
export const gotoBookings = () => goto("/bookings")
export const gotoPrices = () => goto("/prices")

export const goBack = (route = "/") => {
    const ref = document.referrer;
    goto(ref.length > 0 ? ref : route)
}
