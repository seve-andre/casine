<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { page } from "$app/stores"
  import { invalidateAll } from "$app/navigation"
  import { NewGroupSchema, NewGuestSchema } from "~/models"
  import { FirstStepSchema } from "./first-step"
  import { SecondStepSchema } from "./second-step"
  import { secondStepErrorsDefaults, type SecondStepErrors } from "./second-step-errors"
  import { Helper, Input, Label, PaginationItem, StepIndicator } from "flowbite-svelte"
  import { LeftArrow, RightArrow } from "~/lib/ui-components"

  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  $: nextLabel = currentStep === 1 ? "Avanti" : "Fine"
  $: nextAction = currentStep === 1 ? nextStep : onStepDone

  const prevStep = () => (currentStep -= 1)
  const nextStep = () => {
    const firstStepResult = FirstStepSchema.safeParse({
      startDate,
      endDate,
    })

    if (firstStepResult.success) {
      currentStep += 1
      firstStepError = undefined
    } else {
      firstStepError = firstStepResult.error.format()._errors.at(0)
    }
  }

  const onStepDone = () => {
    const secondStepResult = SecondStepSchema.safeParse({
      guest: {
        first_name: firstName,
        last_name: lastName,
        birth_date: birthDate,
      },
    })

    if (secondStepResult.success) {
      invoke("open_apartment", {
        apartmentId: +$page.params.id,
        startDate: startDate,
        endDate: endDate,
        newGuest: NewGuestSchema.parse({
          first_name: firstName,
          last_name: lastName,
          birth_date: birthDate,
        }),
        newGroup: NewGroupSchema.parse({
          nickname: lastName,
        }),
      })

      invalidateAll()
    } else {
      const formattedErrors = secondStepResult.error.format().guest

      secondStepErrors = {
        onFirstName: formattedErrors?.first_name?._errors.at(0),
        onLastName: formattedErrors?.last_name?._errors.at(0),
        onBirthDate: formattedErrors?.birth_date?._errors.at(0),
      }
    }
  }

  // 1st step
  let firstStepError: string | undefined = undefined
  let startDate = ""
  let endDate = ""

  // 2nd step
  let secondStepErrors: SecondStepErrors = {
    ...secondStepErrorsDefaults,
  }
  let firstName = ""
  let lastName = ""
  let birthDate = ""
</script>

<div class="flex flex-col gap-2">
  <StepIndicator {currentStep} {steps} />

  <!-- content belonging to step -->
  <div>
    {#if currentStep === 1}
      <div>
        <Label for="start-date" color={firstStepError ? "red" : "gray"} class="block mb-2">Giorno di arrivo</Label>
        <Input type="date" id="start-date" bind:value={startDate} color={firstStepError ? "red" : "base"} />
        {#if firstStepError}
          <Helper class="mt-2" color="red">{firstStepError}</Helper>
        {/if}
      </div>

      <div>
        <Label for="end-date" color="gray" class="block mb-2">Giorno di partenza</Label>
        <Input type="date" id="end-date" bind:value={endDate} color="base" />
      </div>
    {:else}
      <div class="grid gap-6 md:grid-cols-2">
        <div>
          <Label for="first-name" color={secondStepErrors.onFirstName ? "red" : "gray"} class="block mb-2">Nome</Label>
          <Input
            id="first-name"
            bind:value={firstName}
            color={secondStepErrors.onFirstName ? "red" : "base"}
            placeholder="Andrea"
          />
          {#if secondStepErrors.onFirstName}
            <Helper class="mt-2" color="red">Il nome è obbligatorio</Helper>
          {/if}
        </div>

        <div>
          <Label for="last-name" color={secondStepErrors.onLastName ? "red" : "gray"} class="block mb-2">Cognome</Label>
          <Input
            id="last-name"
            bind:value={lastName}
            color={secondStepErrors.onLastName ? "red" : "base"}
            placeholder="Severi"
          />
          {#if secondStepErrors.onLastName}
            <Helper class="mt-2" color="red">Il cognome è obbligatorio</Helper>
          {/if}
        </div>

        <div>
          <Label for="birth-date" color={secondStepErrors.onBirthDate ? "red" : "gray"} class="block mb-2">
            Data di nascita
          </Label>
          <Input
            type="date"
            id="birth-date"
            bind:value={birthDate}
            color={secondStepErrors.onBirthDate ? "red" : "base"}
          />
          {#if secondStepErrors.onBirthDate}
            <Helper class="mt-2" color="red">La data di nascita è obbligatoria</Helper>
          {/if}
        </div>
      </div>
    {/if}
  </div>

  <div class="flex space-x-3">
    {#if currentStep != 1}
      <PaginationItem class="flex items-center" on:click={prevStep}>
        <LeftArrow clazz="mr-2 w-5 h-5" />
        Torna a scelta periodo
      </PaginationItem>
    {/if}
    <PaginationItem class="flex items-center" on:click={nextAction}>
      {nextLabel}
      <RightArrow clazz="ml-2 w-5 h-5" />
    </PaginationItem>
  </div>
</div>
