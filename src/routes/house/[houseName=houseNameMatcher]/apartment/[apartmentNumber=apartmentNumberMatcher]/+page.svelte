<script lang="ts">
  import { page } from "$app/stores"
  import FilledButton from "@/lib/ui-components/button/FilledButton.svelte"

  import {
    Button,
    Search,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    StepIndicator,
    Datepicker,
    Heading,
    PaginationItem,
    P,
  } from "flowbite-svelte"

  let searchTerm = ""
  let items = [
    { id: 1, maker: "Toyota", type: "ABC", make: 2017 },
    { id: 2, maker: "Ford", type: "CDE", make: 2018 },
    { id: 3, maker: "Volvo", type: "FGH", make: 2019 },
    { id: 4, maker: "Saab", type: "IJK", make: 2020 },
  ]
  $: filteredItems = items.filter(item => item.maker.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1)

  let guests = []

  // guests == 0 data
  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  const prevStep = () => (currentStep -= 1)
  const nextStep = () => (currentStep += 1)
  const openApartment = () => {}

  const stepsTest = {
    1: {
      title: "1 - Scegli periodo",
      content: "",
      nextLabel: "Avanti",
      nextAction: nextStep,
    },
    2: {
      title: "2 - Aggiungi capogruppo",
      content: "",
      nextLabel: "Fine",
      nextAction: openApartment,
    },
  }

  // guests > 0 data
  let tableHeads = ["Nome", "Cognome", "Data di nascita"]
  let tableRows = [
    {
      firstName: "Andrea",
      lastName: "Severi",
      birthday: "02/10/2000",
    },
  ]
</script>

<div class="h-screen flex flex-col">
  <div class="flex-initial">
    <Heading customSize="text-3xl font-bold">Casa A - Appartamento {$page.params.apartmentNumber}</Heading>
  </div>

  <div class="flex-auto">
    {#if guests.length == 0}
      <StepIndicator {currentStep} {steps} />

      <!-- content belonging to step -->
      {#if currentStep == 1}
        <Datepicker datepickerTitle="Periodo di permanenza" range datepickerFormat="dd/mm/yyyy" />
      {:else}
        <Button>2</Button>
      {/if}
    {:else}
      <div>
        <Table striped>
          <TableHead>
            {#each tableHeads as head}
              <TableHeadCell>{head}</TableHeadCell>
            {/each}
          </TableHead>
          <TableBody>
            {#each tableRows as row}
              <TableBodyRow>
                <TableBodyCell>{row.firstName}</TableBodyCell>
                <TableBodyCell>{row.lastName}</TableBodyCell>
                <TableBodyCell>{row.birthday}</TableBodyCell>
              </TableBodyRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>

  <div class="flex-auto">
    <div class="flex space-x-3">
      {#if currentStep != 1}
        <PaginationItem class="flex items-center" on:click={prevStep}>
          <svg class="mr-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path
              fill-rule="evenodd"
              d="M7.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l2.293 2.293a1 1 0 010 1.414z"
              clip-rule="evenodd"
            />
          </svg>
          Torna a scelta periodo
        </PaginationItem>
      {/if}
      <PaginationItem class="flex items-center" on:click={currentStep == 1 ? nextStep : openApartment}>
        {#if currentStep == 1}
          Avanti
        {:else}
          Fine
        {/if}
        <svg class="ml-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
          <path
            fill-rule="evenodd"
            d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z"
            clip-rule="evenodd"
          />
        </svg>
      </PaginationItem>
    </div>
  </div>
</div>
