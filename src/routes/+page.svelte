<script lang="ts">
  import { Heading, Tooltip } from "flowbite-svelte"

  import { FilledButton } from "~/lib/ui-components"

  import type { PageData } from "./$types"

  export let data: PageData
</script>

<div class="h-full flex flex-col gap-4 py-5 px-10">
  <div class="flex flex-col gap-12">
    {#each data.housesWithApartments as { house, apartments }}
      <div class="flex flex-col gap-12">
        <Heading customSize="font-bold text-2xl">Casa {house.house_name}</Heading>

        <!-- show 3 buttons per line -->
        <!-- width at 75%, so buttons don't look "weird" -->
        <div class="grid grid-cols-3 gap-6 w-3/4">
          {#each apartments as apartment}
            <FilledButton href={`apartments/${apartment.id}`}>
              App. {apartment.apartment_number}
            </FilledButton>
            <Tooltip placement="bottom">Vai all'appartamento {apartment.apartment_number}</Tooltip>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>
