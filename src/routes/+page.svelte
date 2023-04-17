<script lang="ts">
  import type { PageData } from "./$types"
  import { Heading, Tooltip } from "flowbite-svelte"
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"
  import NewGuestForm from "~/lib/page-components/common/NewGuestForm.svelte"

  export let data: PageData
</script>

<div class="home-container">
  <div class="apartment-selection flex flex-col gap-4">
    <Heading customSize="text-2xl font-bold">Casa A</Heading>
    <div class="grid grid-cols-3 gap-4">
      {#each data.apartments as apartment}
        <FilledButton href={`house/${apartment.house_name}/apartment/${apartment.apartment_number}`}>
          App. {apartment.apartment_number}
        </FilledButton>
        <Tooltip placement="bottom">Vai all'appartamento {apartment.apartment_number}</Tooltip>
      {/each}
    </div>
  </div>
  <div class="bookings-utils flex flex-col gap-4">
    <Heading tag="h2" customSize="text-xl font-semibold">Utili</Heading>
    <div class="grid grid-cols-3 gap-4">
      <FilledButton href="/bookings">Prenotazioni</FilledButton>
      <Tooltip placement="bottom">Vedi prenotazioni</Tooltip>

      <FilledButton href="/prices">Prezzi</FilledButton>
      <Tooltip placement="bottom">Vedi prezzi</Tooltip>
    </div>
  </div>
  <div class="form">
    <NewGuestForm apartments={data.apartments} />
  </div>
</div>

<style>
  .home-container {
    display: grid;
    grid-template-columns: 1fr 1.5fr;
    grid-template-rows: 1.5fr 1fr;
    grid-auto-flow: row;
    grid-template-areas:
      "apartment-selection form"
      "bookings-utils form";
    height: 100%;
  }

  .apartment-selection {
    grid-area: apartment-selection;
  }

  .bookings-utils {
    grid-area: bookings-utils;
  }

  .form {
    grid-area: form;
    padding-left: 5%;
  }
</style>
