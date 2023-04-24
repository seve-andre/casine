<script lang="ts">
  import { Helper, Input, Label, PaginationItem, StepIndicator } from "flowbite-svelte"
  import { LeftArrow, RightArrow } from "~/lib/ui-components"
  import { FirstStepSchema } from "./stepper/first-step"

  export let onDone: () => void

  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  const prevStep = () => (currentStep -= 1)
  const nextStep = () => {
    const firstStepResult = FirstStepSchema.safeParse({
      startDate,
      endDate,
    })

    if (firstStepResult.success) {
      currentStep += 1
    } else {
      firstStepError = firstStepResult.error.flatten().fieldErrors.startDate?.at(0)
    }
  }

  $: nextLabel = currentStep === 1 ? "Avanti" : "Fine"
  $: nextAction = currentStep === 1 ? nextStep : onDone

  // 1st step
  let firstStepError: string | undefined = undefined
  let startDate = ""
  let endDate = ""

  // 2nd step
  let secondStepError: string | undefined = undefined
  let firstName = ""
</script>

<div class="flex flex-col gap-2">
  <StepIndicator {currentStep} {steps} />

  <!-- content belonging to step -->
  <div>
    {#if currentStep === 1}
      <div>
        <Label for="start-date" color={firstStepError ? "red" : "gray"} class="block mb-2">Giorno di arrivo</Label>
        <Input
          id="start-date"
          color={firstStepError ? "red" : "base"}
          type="date"
          placeholder="Scegli giorno di arrivo"
          bind:value={startDate}
        />
        {#if firstStepError}
          <Helper class="mt-2" color="red">{firstStepError}</Helper>
        {/if}
      </div>

      <div>
        <Label for="end-date" color={firstStepError ? "red" : "gray"} class="block mb-2">Giorno di partenza</Label>
        <Input
          id="end-date"
          color={firstStepError ? "red" : "base"}
          type="date"
          placeholder="Scegli giorno di partenza"
          bind:value={endDate}
        />
        {#if firstStepError}
          <Helper class="mt-2" color="red">{firstStepError}</Helper>
        {/if}
      </div>
    {:else}
      <div class="grid gap-6 md:grid-cols-2">
        <div>
          <Label for="first-name" color="gray" class="block mb-2">Nome</Label>
          <Input id="first-name" bind:value={firstName} color="base" placeholder="Andrea" />
          {#if secondStepError}
            <Helper class="mt-2" color="red">Il nome è obbligatorio</Helper>
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
