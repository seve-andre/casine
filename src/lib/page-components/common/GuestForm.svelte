<script lang="ts">
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"
  import { useValidation } from "~/routes/apartments/[id=apartment_id_matcher]/components/stepper/validation"

  import { guestFormErrorsDefaults, type GuestFormErrors } from "./guest-form-errors"
  import GuestFormData from "./GuestFormData.svelte"

  import { invalidateAll } from "$app/navigation"

  // props
  export let onSubmitSuccess: () => void

  // data
  export let firstName: string
  export let lastName: string
  export let birthDate: string

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
      birth_date: birthDate
    })

    if (guestResult.success) {
      onSubmitSuccess()
      invalidateAll()
    } else {
      const formattedErrors = guestResult.error.format()

      errors = {
        onFirstName: formattedErrors.first_name?._errors.at(0),
        onLastName: formattedErrors.last_name?._errors.at(0),
        onBirthDate: formattedErrors.birth_date?._errors.at(0)
      }
    }
  }
</script>

<form on:submit|preventDefault={onSubmit} class="form-container">
  <div class="form__filling">
    <GuestFormData bind:firstName bind:lastName bind:birthDate bind:errors />
  </div>

  <div class="form__controls">
    <FilledButton type="submit">Aggiungi</FilledButton>
  </div>
</form>

<style>
  .form-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .form__controls {
    display: flex;
    justify-content: flex-end;
  }
</style>
