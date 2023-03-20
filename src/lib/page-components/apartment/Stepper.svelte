<script lang="ts">
  import LeftArrow from "@/lib/ui-components/icon/LeftArrow.svelte"
  import RightArrow from "@/lib/ui-components/icon/RightArrow.svelte"
  import { Button, Helper, Input, Label, PaginationItem, StepIndicator } from "flowbite-svelte"

  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  const prevStep = () => (currentStep -= 1)
  const nextStep = () => {
    if (datesAreCorrect) {
      console.log("greater")
      currentStep += 1
    } else {
      console.log(`less - ${startDate} < ${endDate}}`)
    }
  }
  export let onDone: () => void

  // const createGuest
  $: nextLabel = currentStep == 1 ? "Avanti" : "Fine"
  $: nextAction = currentStep == 1 ? nextStep : onDone

  const stepsTest = [
    {
      name: "1 - Scegli periodo",
      nextLabel: "Avanti",
      nextAction: nextStep,
    },
    {
      name: "2 - Aggiungi capogruppo",
      nextLabel: "Fine",
      nextAction: onDone,
    },
  ]

  let startDate: string = ""
  let endDate: string = ""
  $: isStartDateCorrect = startDate != ""
  $: isEndDateCorrect = endDate != ""
  $: datesAreCorrect = isStartDateCorrect && isEndDateCorrect && Date.parse(startDate) < Date.parse(endDate)
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
      <Button>2</Button>
    {/if}
  </div>

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
