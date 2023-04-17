
const minApartmentNumber = 1
const maxApartmentNumber = 6

export const Constants = {
    houseRegex: /A|B/,
    apartmentRegex: new RegExp(`/[${minApartmentNumber}-${maxApartmentNumber}]/`),
    minApartmentNumber: minApartmentNumber,
    maxApartmentNumber: maxApartmentNumber
} as const
