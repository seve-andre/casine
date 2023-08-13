<script lang="ts">
  import { Helper, Input, Label } from "flowbite-svelte"

  import type { FirstStepErrors } from "./first-step"

  // data
  export let startDate: string
  export let endDate: string

  // errors
  export let errors: FirstStepErrors
</script>

<div>
  <Label for="start-date" color={errors.onStartDate ? "red" : "gray"} class="block mb-2"
    >Giorno di arrivo</Label
  >
  <Input
    type="date"
    id="start-date"
    bind:value={startDate}
    color={errors.onStartDate ? "red" : "base"}
  />
  {#if errors.onStartDate}
    <Helper class="mt-2" color="red">{errors.onStartDate}</Helper>
  {/if}
</div>

<div>
  <Label for="end-date" color={errors.onEndDate ? "red" : "gray"} class="block mb-2"
    >Giorno di partenza</Label
  >
  <Input type="date" id="end-date" bind:value={endDate} color={errors.onEndDate ? "red" : "base"} />
  {#if errors.onEndDate}
    <Helper class="mt-2" color="red">{errors.onEndDate}</Helper>
  {/if}
</div>
