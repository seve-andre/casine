// these are harcoded since there's a finite number of apartments
const minApartmentNumber = 1
const maxApartmentNumber = 6
const minApartmentId = 1
const maxApartmentId = 12

export const Constants = {
    houseRegex: /A|B/,
    apartmentRegex: new RegExp(`[${minApartmentId}-${maxApartmentId}]`),
    minApartmentNumber: minApartmentNumber,
    maxApartmentNumber: maxApartmentNumber,
    minApartmentId: minApartmentId,
    maxApartmentId: maxApartmentId
} as const
