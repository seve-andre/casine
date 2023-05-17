<script lang="ts">
  import { Helper, Input, Label } from "flowbite-svelte"
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"
  import { guestFormErrorsDefaults, type GuestFormErrors } from "./guest-form-errors"
  import { useValidation } from "~/routes/apartments/[id=apartment_id_matcher]/components/stepper/validation"

  // data
  let firstName: string
  let lastName: string
  let birthDate: string

  // errors
  let errors: GuestFormErrors = {
    ...guestFormErrorsDefaults,
  }

  // submission
  const { validateNewGuest } = useValidation()
  const onSubmit = () => {
    const guestResult = validateNewGuest({
      first_name: firstName,
      last_name: lastName,
      birth_date: birthDate,
    })

    if (guestResult.success) {
      console.log("successo")
    } else {
      const formattedErrors = guestResult.error.format()

      errors = {
        onFirstName: formattedErrors.first_name?._errors.at(0),
        onLastName: formattedErrors.last_name?._errors.at(0),
        onBirthDate: formattedErrors.birth_date?._errors.at(0),
      }
    }
  }
</script>

<form on:submit|preventDefault={onSubmit} class="form-container">
  <div class="grid gap-6 md:grid-cols-2">
    <div>
      <Label for="first-name" color={errors.onFirstName ? "red" : "gray"} class="block mb-2">Nome</Label>
      <Input id="first-name" bind:value={firstName} color={errors.onFirstName ? "red" : "base"} placeholder="Andrea" />
      {#if errors.onFirstName}
        <Helper class="mt-2" color="red">{errors.onFirstName}</Helper>
      {/if}
    </div>

    <div>
      <Label for="last-name" color={errors.onLastName ? "red" : "gray"} class="block mb-2">Cognome</Label>
      <Input id="last-name" bind:value={lastName} color={errors.onLastName ? "red" : "base"} placeholder="Severi" />
      {#if errors.onLastName}
        <Helper class="mt-2" color="red">{errors.onLastName}</Helper>
      {/if}
    </div>

    <div>
      <Label for="birth-date" color={errors.onBirthDate ? "red" : "gray"} class="block mb-2">Data di nascita</Label>
      <Input type="date" id="birth-date" bind:value={birthDate} color={errors.onBirthDate ? "red" : "base"} />
      {#if errors.onBirthDate}
        <Helper class="mt-2" color="red">{errors.onBirthDate}</Helper>
      {/if}
    </div>
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
