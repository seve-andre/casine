<script lang="ts">
  import { Helper, Input, Label } from "flowbite-svelte"

  // data
  export let startDate: string
  export let endDate: string

  // errors
  export let error: string | undefined
</script>

<div>
  <Label for="start-date" color={error ? "red" : "gray"} class="block mb-2">Giorno di arrivo</Label>
  <Input type="date" id="start-date" bind:value={startDate} color={error ? "red" : "base"} />
  {#if error}
    <Helper class="mt-2" color="red">{error}</Helper>
  {/if}
</div>

<div>
  <Label for="end-date" color="gray" class="block mb-2">Giorno di partenza</Label>
  <Input type="date" id="end-date" bind:value={endDate} color="base" />
</div>
