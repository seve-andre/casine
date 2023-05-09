<script lang="ts">
  import type { PageData } from "./$types"
  import { Heading, Tooltip } from "flowbite-svelte"
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"

  export let data: PageData
</script>

<div class="flex flex-col gap-4 p-10">
  <Heading customSize="text-2xl font-bold">Casa A</Heading>
  <div class="grid grid-cols-3 gap-10">
    {#each data.apartments as apartment}
      <FilledButton href={`apartments/${apartment.id}`}>
        App. {apartment.apartment_number}
      </FilledButton>
      <Tooltip placement="bottom">Vai all'appartamento {apartment.apartment_number}</Tooltip>
    {/each}
  </div>
</div>
