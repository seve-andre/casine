<script lang="ts">
  import { page } from "$app/stores"
  import GuestsTable from "@/lib/page-components/apartment/GuestsTable.svelte"
  import Stepper from "@/lib/page-components/apartment/Stepper.svelte"
  import { Heading } from "flowbite-svelte"

  let isApartmentEmpty = true
</script>

<div class="h-screen flex flex-col gap-4">
  <div class="flex-initial">
    <Heading customSize="text-3xl font-bold">
      Casa {$page.params.houseName} - Appartamento {$page.params.apartmentNumber}
    </Heading>
  </div>

  <div class="flex-auto">
    {#if isApartmentEmpty}
      <Stepper onDone={() => (isApartmentEmpty = !isApartmentEmpty)} />
    {:else}
      <GuestsTable />
    {/if}
  </div>
</div>
