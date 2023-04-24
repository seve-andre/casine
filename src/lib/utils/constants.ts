export const Constants = {
    houseRegex: /A|B/,
    apartmentRegex: /[1-9]|1[0-2]/,
    // these are harcoded since there's a finite number of apartments
    minApartmentId: 1,
    maxApartmentId: 12,
    minApartmentNumber: 1,
    maxApartmentNumber: 6,

    // yyyy-mm-dd
    dateRegex: /\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])/m
} as const
