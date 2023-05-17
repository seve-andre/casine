export type GuestFormErrors = {
    onFirstName?: string
    onLastName?: string
    onBirthDate?: string
}

export const guestFormErrorsDefaults: Pick<GuestFormErrors, "onFirstName" | "onLastName" | "onBirthDate"> = {
    onFirstName: undefined,
    onLastName: undefined,
    onBirthDate: undefined
}
