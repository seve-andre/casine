<script lang="ts">
  import type { PageData } from "./$types"
  import { Heading } from "flowbite-svelte"
  import { Stepper, GuestsTable } from "~/lib/page-components/apartment"

  export let data: PageData
  let isApartmentEmpty = data.guests.length === 0
</script>

<div class="h-screen flex flex-col gap-4">
  <div class="flex-initial">
    <Heading customSize="text-3xl font-bold">{data.title}</Heading>
  </div>

  <div class="flex-auto">
    {#if isApartmentEmpty}
      <Stepper onDone={() => (isApartmentEmpty = false)} />
    {:else}
      <GuestsTable rent={data.rent} guests={data.guests} />
    {/if}
  </div>
</div>
