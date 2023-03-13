<script lang="ts">
  import { Heading, Tooltip } from "flowbite-svelte"
  import { goto } from "$app/navigation"
  import FilledButton from "@/lib/ui-components/button/FilledButton.svelte"
  import type { Apartment } from "@/models/Apartment"
  import { gotoApartmentInHouse, gotoBookings, gotoPrices } from "./navigation"

  let apartmentsHouseA: Apartment[] = [
    {
      house_id: 1,
      apartment_number: 1,
    },
    {
      house_id: 1,
      apartment_number: 2,
    },
    {
      house_id: 1,
      apartment_number: 3,
    },
    {
      house_id: 1,
      apartment_number: 4,
    },
    {
      house_id: 1,
      apartment_number: 5,
    },
    {
      house_id: 1,
      apartment_number: 6,
    },
  ]
</script>

<div class="home-container">
  <div class="apartment-selection flex flex-col gap-4">
    <Heading customSize="text-2xl font-bold">Casa A</Heading>
    <div class="grid grid-cols-3 gap-4">
      {#each apartmentsHouseA as apartment}
        <FilledButton on:click={() => gotoApartmentInHouse(apartment.apartment_number, "A")}>
          App. {apartment.apartment_number}
        </FilledButton>
        <Tooltip placement="bottom">Vai all'appartamento {apartment.apartment_number}</Tooltip>
      {/each}
    </div>
  </div>
  <div class="bookings-utils flex flex-col gap-4">
    <Heading tag="h2" customSize="text-xl font-semibold">Utili</Heading>
    <div class="grid grid-cols-3 gap-4">
      <FilledButton on:click={() => gotoBookings()}>Prenotazioni</FilledButton>
      <Tooltip placement="bottom">Vedi prenotazioni</Tooltip>

      <FilledButton on:click={() => gotoPrices()}>Prezzi</FilledButton>
      <Tooltip placement="bottom">Vedi prezzi</Tooltip>
    </div>
  </div>
  <div class="form" />
</div>

<style>
  .home-container {
    display: grid;
    grid-template-columns: 1fr 1.5fr;
    grid-template-rows: 1.5fr 1fr;
    gap: 0px 0px;
    grid-auto-flow: row;
    grid-template-areas:
      "apartment-selection form"
      "bookings-utils .";
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
  }
</style>
