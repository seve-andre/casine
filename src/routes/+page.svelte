<script lang="ts">
  import { gotoApartmentInHouse, gotoBookings, gotoPrices } from "./navigation-utils"
  import { Button, Heading, Tooltip } from "flowbite-svelte"
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"
  import type { Apartment } from "~/models/Apartment"
  import NewGuestForm from "~/lib/page-components/NewGuestForm.svelte"
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte"

  let apartments: Apartment[] = [
    {
      id: 1,
      house_name: "A",
      apartment_number: 1,
    },
    {
      id: 2,
      house_name: "A",
      apartment_number: 2,
    },
    {
      id: 3,
      house_name: "A",
      apartment_number: 3,
    },
    {
      id: 4,
      house_name: "A",
      apartment_number: 4,
    },
    {
      id: 5,
      house_name: "A",
      apartment_number: 5,
    },
    {
      id: 6,
      house_name: "A",
      apartment_number: 6,
    },
  ]

  // apartments grouped by house_name
  let apartmentsByHouse = apartments.reduce((r, ap) => {
    r[ap.house_name] = r[ap.house_name] || []
    r[ap.house_name].push(ap.apartment_number)
    return r
  }, Object.create(null))

  let apartmentHouseA = apartments.filter(a => a.house_name == "A")

  // let apartmentHouseA: Apartment[]

  onMount(async () => {
    // apartmentHouseA = await invoke("get_apartments")
  })
</script>

<div class="home-container">
  <div class="apartment-selection flex flex-col gap-4">
    <Heading customSize="text-2xl font-bold">Casa A</Heading>
    <div class="grid grid-cols-3 gap-4">
      {#each apartmentHouseA as apartment}
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
      <FilledButton on:click={() => gotoBookings()}>Prenotazioni</FilledButton>
      <Tooltip placement="bottom">Vedi prenotazioni</Tooltip>

      <FilledButton on:click={() => gotoPrices()}>Prezzi</FilledButton>
      <Tooltip placement="bottom">Vedi prezzi</Tooltip>
    </div>
  </div>
  <div class="form">
    <NewGuestForm />
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
