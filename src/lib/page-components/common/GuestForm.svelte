<script lang="ts">
  import FilledButton from "$lib/ui-components/buttons/FilledButton.svelte"


  import { guestFormErrorsDefaults, type GuestFormErrors } from "./guest-form-errors"
  import GuestFormData from "./GuestFormData.svelte"

  import { invalidateAll } from "$app/navigation"
  import { useValidation } from "~/routes/apartments/[id=apartment_id_matcher]/components/stepper/validation"

  // props
  export let onSubmitSuccess: () => void

  // data
  export let firstName: string
  export let lastName: string
  export let birthdate: string

  // errors
  let errors: GuestFormErrors = {
    ...guestFormErrorsDefaults
  }

  // submission
  const { validateNewGuest } = useValidation()
  const onSubmit = () => {
    const guestResult = validateNewGuest({
      first_name: firstName,
      last_name: lastName,
      birth_date: birthdate
    })

    if (guestResult.success) {
      onSubmitSuccess()
      invalidateAll()
    } else {
      const formattedErrors = guestResult.error.format()

      errors = {
        onFirstName: formattedErrors.first_name?._errors.at(0),
        onLastName: formattedErrors.last_name?._errors.at(0),
        onBirthdate: formattedErrors.birth_date?._errors.at(0)
      }
    }
  }
</script>

<form on:submit|preventDefault={onSubmit} class="flex flex-col gap-4">
  <GuestFormData bind:firstName bind:lastName bind:birthdate bind:errors />
  <div class="flex justify-end items-center">
    <FilledButton type="submit">Aggiungi</FilledButton>
  </div>
</form>
