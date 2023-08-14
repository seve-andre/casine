<script lang="ts">
  import { Heading } from "flowbite-svelte"

  import type { PageData } from "./$types"
  import Stepper from "./components/stepper/Stepper.svelte"
  import GuestsTable from "./components/table/GuestsTable.svelte"

  export let data: PageData
  $: isApartmentEmpty = data.guests === null
</script>

<div class="h-full flex flex-col gap-4 py-5 px-10">
  <div class="flex-initial">
    <Heading customSize="font-bold text-3xl">{data.title}</Heading>
  </div>

  <div class="flex-auto">
    {#if isApartmentEmpty}
      <Stepper />
    {:else}
      <!-- table is recreated when guests change -->
      {#key data.guests}
        {#if data.guests && data.rent && data.group}
          <GuestsTable rent={data.rent} guests={data.guests} group={data.group} />
        {/if}
      {/key}
    {/if}
  </div>
</div>
