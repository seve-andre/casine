export const Constants = {
    houseRegex: /A|B/,
    apartmentRegex: /[1-9]|1[0-2]/,
    // these are harcoded since there's a finite number of apartments
    minApartmentId: 1,
    maxApartmentId: 12,
    minApartmentNumber: 1,
    maxApartmentNumber: 6
} as const
