<script lang="ts">
  import type { PageData } from "./$types"
  import { Heading } from "flowbite-svelte"
  import Stepper from "./components/stepper/Stepper.svelte"
  import GuestsTable from "./components/table/GuestsTable.svelte"

  export let data: PageData
  $: isApartmentEmpty = data.guests.length === 0
</script>

<div class="h-full flex flex-col gap-4 p-10">
  <div>
    <Heading customSize="text-3xl font-bold">{data.title}</Heading>
  </div>

  <div>
    {#if isApartmentEmpty}
      <Stepper />
    {:else}
      <GuestsTable rent={data.rent} guests={data.guests} group={data.group} />
    {/if}
  </div>
</div>
