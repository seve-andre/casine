<script lang="ts">
  import type { PageData } from "./$types"
  import { Heading } from "flowbite-svelte"
  import Stepper from "./components/stepper/Stepper.svelte"
  import GuestsTable from "./components/table/GuestsTable.svelte"

  export let data: PageData
  $: isApartmentEmpty = data.guests.length === 0
</script>

<div class="apartment">
  <div class="apartment__title">
    <Heading customSize="apartment__title__text title-text">{data.title}</Heading>
  </div>

  <div class="apartment__content">
    {#if isApartmentEmpty}
      <Stepper />
    {:else}
      {#key data.guests}
        <GuestsTable rent={data.rent} guests={data.guests} group={data.group} />
      {/key}
    {/if}
  </div>
</div>

<style>
  .apartment {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.25rem 2.5rem;
  }

  .apartment__title {
    flex: 0 1 10%;
  }

  :global(.apartment__title__text) {
    font-size: 1.875rem;
    line-height: 2.25rem;
  }

  .apartment__content {
    flex: 1 1 90%;
  }
</style>
