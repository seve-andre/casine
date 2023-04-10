<script lang="ts">
  import LeftArrow from "~/lib/ui-components/icon/LeftArrow.svelte"
  import RightArrow from "~/lib/ui-components/icon/RightArrow.svelte"
  import { Helper, Input, Label, PaginationItem, StepIndicator } from "flowbite-svelte"

  export let onDone: () => void

  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  const prevStep = () => (currentStep -= 1)
  const nextStep = () => {
    if (datesAreCorrect) {
      currentStep += 1
    }
  }

  $: nextLabel = currentStep == 1 ? "Avanti" : "Fine"
  $: nextAction = currentStep == 1 ? nextStep : onDone

  let startDate: string = ""
  let endDate: string = ""
  $: isStartDateCorrect = startDate != ""
  $: isEndDateCorrect = endDate != ""
  $: datesAreCorrect = isStartDateCorrect && isEndDateCorrect && Date.parse(startDate) < Date.parse(endDate)

  let firstName: string | undefined = undefined
  $: isFirstNameEmpty = firstName == undefined || firstName == ""
  let firstNameTouched = false
</script>

<div class="flex flex-col gap-2">
  <StepIndicator {currentStep} {steps} />

  <!-- content belonging to step -->
  <div>
    {#if currentStep == 1}
      <div>
        <Label for="start-date" color={datesAreCorrect ? "gray" : "red"} class="block mb-2">Giorno di arrivo</Label>
        <Input
          id="start-date"
          color={datesAreCorrect ? "base" : "red"}
          type="date"
          placeholder="Scegli giorno di arrivo"
          bind:value={startDate}
        />
        {#if !datesAreCorrect}
          <Helper class="mt-2" color="red">La data di arrivo deve essere prima della data di partenza</Helper>
        {/if}
      </div>

      <div>
        <Label for="end-date" color={datesAreCorrect ? "gray" : "red"} class="block mb-2">Giorno di partenza</Label>
        <Input
          id="end-date"
          color={datesAreCorrect ? "base" : "red"}
          type="date"
          placeholder="Scegli giorno di partenza"
          bind:value={endDate}
        />
        {#if !datesAreCorrect}
          <Helper class="mt-2" color="red">La data di arrivo deve essere prima della data di partenza</Helper>
        {/if}
      </div>
    {:else}
      <div class="grid gap-6 md:grid-cols-2">
        <div>
          <Label for="first-name" color={isFirstNameEmpty && firstNameTouched ? "red" : "gray"} class="block mb-2">
            Nome
          </Label>
          <Input
            id="first-name"
            bind:value={firstName}
            color={isFirstNameEmpty && firstNameTouched ? "red" : "base"}
            placeholder="Andrea"
            on:keydown={() => (firstNameTouched = true)}
          />
          {#if isFirstNameEmpty && firstNameTouched}
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
