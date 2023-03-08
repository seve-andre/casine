<script lang="ts">
  import { page } from "$app/stores"

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
  const nextStep = () => (currentStep += 1)

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

{#if guests.length == 0}
  <StepIndicator {currentStep} {steps} />

  <!-- content belonging to step -->
  {#if currentStep == 1}
    <Datepicker datepickerTitle="My Vacation" range datepickerFormat="dd/mm/yyyy" />
  {:else}
    <Button>2</Button>
  {/if}

  <!-- button prev - next -->
  <Button on:click={nextStep} disabled={currentStep == 2}>Avanti</Button>
{:else}
  <div>
    <Search>
      <Button>Search</Button>
    </Search>

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
