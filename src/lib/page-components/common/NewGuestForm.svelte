<script lang="ts">
  import { Heading, Helper, Input, Label, Select } from "flowbite-svelte"
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"
  import OutlinedButton from "~/lib/ui-components/button/OutlinedButton.svelte"
  import { NewGuestSchema, type Apartment } from "~/models"
  import {
    secondStepErrorsDefaults,
    type SecondStepErrors,
  } from "~/routes/apartment/[id=apartment_id_matcher]/components/stepper/second-step-errors"

  export let apartments: Apartment[]

  let selectedApartmentId = apartments[0].id
  const apartmentsNames = apartments.map(a => ({
    value: a.id,
    name: `App. ${a.apartment_number}`,
  }))

  // guest data
  let formErrors: SecondStepErrors = {
    ...secondStepErrorsDefaults,
  }
  let firstName = ""
  let lastName = ""
  let birthDate = ""

  // all of the fields are different from empty
  $: isFormValid = firstName && lastName && birthDate
  $: canReset = firstName || lastName || birthDate

  const onAddNewGuest = () => {
    const newGuestResult = NewGuestSchema.safeParse({
      first_name: firstName,
      last_name: lastName,
      birth_date: birthDate,
    })

    if (newGuestResult.success) {
    } else {
      const formattedErrors = newGuestResult.error.format()

      formErrors = {
        onFirstName: formattedErrors.first_name?._errors.at(0),
        onLastName: formattedErrors?.last_name?._errors.at(0),
        onBirthDate: formattedErrors?.birth_date?._errors.at(0),
      }
    }
  }

  const resetForm = () => {
    firstName = ""
    lastName = ""
    birthDate = ""
  }
</script>

<form class="bg-gray-200 p-5 rounded-lg h-full flex flex-col gap-5">
  <Heading customSize="text-xl font-bold">Aggiungi ospite</Heading>
  <div class="flex-initial">
    <Label>
      Scegli appartamento casa A
      <Select
        class="mt-2"
        items={apartmentsNames}
        bind:value={selectedApartmentId}
        placeholder="Seleziona uno degli appartamenti"
      />
    </Label>
  </div>

  <div class="flex-auto">
    <div class="grid gap-6 md:grid-cols-2">
      <div>
        <Label for="first-name" color={formErrors.onFirstName ? "red" : "gray"} class="block mb-2">Nome</Label>
        <Input
          id="first-name"
          bind:value={firstName}
          color={formErrors.onFirstName ? "red" : "base"}
          placeholder="Andrea"
        />
        {#if formErrors.onFirstName}
          <Helper class="mt-2" color="red">Il nome è obbligatorio</Helper>
        {/if}
      </div>

      <div>
        <Label for="last-name" color={formErrors.onLastName ? "red" : "gray"} class="block mb-2">Cognome</Label>
        <Input
          id="last-name"
          bind:value={lastName}
          color={formErrors.onLastName ? "red" : "base"}
          placeholder="Severi"
        />
        {#if formErrors.onLastName}
          <Helper class="mt-2" color="red">Il cognome è obbligatorio</Helper>
        {/if}
      </div>

      <div>
        <Label for="birth-date" color={formErrors.onBirthDate ? "red" : "gray"} class="block mb-2">
          Data di nascita
        </Label>
        <Input type="date" id="birth-date" bind:value={birthDate} color={formErrors.onBirthDate ? "red" : "base"} />
        {#if formErrors.onBirthDate}
          <Helper class="mt-2" color="red">La data di nascita è obbligatoria</Helper>
        {/if}
      </div>
    </div>
  </div>

  <div class="flex-initial flex justify-end gap-5">
    <OutlinedButton color="red" disabled={!canReset} on:click={() => resetForm()}>Reset</OutlinedButton>
    <FilledButton disabled={!isFormValid} on:click={() => onAddNewGuest()}>Aggiungi</FilledButton>
  </div>
</form>
