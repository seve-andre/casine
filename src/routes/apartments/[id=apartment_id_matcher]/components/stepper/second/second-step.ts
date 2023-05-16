export type SecondStepErrors = {
    onFirstName?: string
    onLastName?: string
    onBirthDate?: string
}

export const secondStepErrorsDefaults: Pick<SecondStepErrors, "onFirstName" | "onLastName" | "onBirthDate"> = {
    onFirstName: undefined,
    onLastName: undefined,
    onBirthDate: undefined
}
