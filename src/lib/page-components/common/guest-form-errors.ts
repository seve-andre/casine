export type GuestFormErrors = {
  onFirstName?: string
  onLastName?: string
  onBirthdate?: string
}

export const guestFormErrorsDefaults: Pick<
  GuestFormErrors,
  "onFirstName" | "onLastName" | "onBirthdate"
> = {
  onFirstName: undefined,
  onLastName: undefined,
  onBirthdate: undefined
}
