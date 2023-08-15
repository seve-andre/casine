<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { StepIndicator } from "flowbite-svelte"

  import { FilledButton, TextButton } from "$lib/ui-components"
  import { NewGroupSchema, NewGuestSchema } from "$models"

  import {
    FirstStepSchema,
    type FirstStepErrors,
    firstStepErrorsDefaults
  } from "./first/first-step"
  import FirstStep from "./first/FirstStep.svelte"
  import { secondStepErrorsDefaults, type SecondStepErrors } from "./second/second-step"
  import SecondStep from "./second/SecondStep.svelte"

  import { invalidateAll } from "$app/navigation"
  import { page } from "$app/stores"

  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  $: nextLabel = currentStep === 1 ? "Avanti" : "Fine"
  $: nextAction = currentStep === 1 ? nextStep : onStepDone

  const prevStep = () => {
    currentStep -= 1
  }
  const nextStep = () => {
    const firstStepResult = FirstStepSchema.safeParse({
      startDate,
      endDate
    })

    if (firstStepResult.success) {
      currentStep += 1

      firstStepErrors = {
        ...firstStepErrorsDefaults
      }
    } else {
      const formattedErrors = firstStepResult.error.format()

      firstStepErrors = {
        onStartDate: formattedErrors.startDate?._errors.at(0),
        onEndDate: formattedErrors.endDate?._errors.at(0)
      }
    }
  }

  const onStepDone = () => {
    const secondStepResult = NewGuestSchema.safeParse({
      first_name: firstName,
      last_name: lastName,
      birth_date: birthDate
    })

    if (secondStepResult.success) {
      invoke("open_apartment", {
        apartmentId: +$page.params.id,
        startDate: startDate,
        endDate: endDate,
        newGuest: NewGuestSchema.parse({
          first_name: firstName,
          last_name: lastName,
          birth_date: birthDate
        }),
        newGroup: NewGroupSchema.parse({
          nickname: lastName
        })
      })

      invalidateAll()
    } else {
      const formattedErrors = secondStepResult.error.format()

      secondStepErrors = {
        onFirstName: formattedErrors?.first_name?._errors.at(0),
        onLastName: formattedErrors?.last_name?._errors.at(0),
        onBirthDate: formattedErrors?.birth_date?._errors.at(0)
      }
    }
  }

  // 1st step
  let firstStepErrors: FirstStepErrors = {
    ...firstStepErrorsDefaults
  }
  let startDate = ""
  let endDate = ""

  // 2nd step
  let secondStepErrors: SecondStepErrors = {
    ...secondStepErrorsDefaults
  }
  let firstName = ""
  let lastName = ""
  let birthDate = ""
</script>

<div class="h-full flex flex-col gap-4">
  <div class="flex-initial">
    <StepIndicator {currentStep} {steps} color="blue" />
  </div>

  <!-- content belonging to step -->
  <div class="flex-auto">
    <form on:submit|preventDefault={nextAction} class="h-full flex flex-col gap-2">
      <div class="flex-auto basis-4/5 flex flex-col gap-4">
        {#if currentStep === 1}
          <FirstStep bind:startDate bind:endDate bind:errors={firstStepErrors} />
        {:else}
          <SecondStep bind:firstName bind:lastName bind:birthDate bind:errors={secondStepErrors} />
        {/if}
      </div>

      <div class="flex-initial basis-1/5 flex justify-end items-center gap-2">
        {#if currentStep !== 1}
          <TextButton on:click={prevStep}>Torna a scelta periodo</TextButton>
        {/if}

        <FilledButton type="submit">{nextLabel}</FilledButton>
      </div>
    </form>
  </div>
</div>
