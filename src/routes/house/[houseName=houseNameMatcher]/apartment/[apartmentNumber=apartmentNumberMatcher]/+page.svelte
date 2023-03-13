<script lang="ts">
  import { page } from "$app/stores"
  import LeftArrow from "@/lib/ui-components/icon/LeftArrow.svelte"
  import RightArrow from "@/lib/ui-components/icon/RightArrow.svelte"
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    StepIndicator,
    Heading,
    PaginationItem,
  } from "flowbite-svelte"

  let guests = [1]

  // guests == 0 data
  let currentStep = 1
  let steps = ["1 - Scegli periodo", "2 - Aggiungi capogruppo"]
  const prevStep = () => (currentStep -= 1)
  const nextStep = () => (currentStep += 1)
  const openApartment = () => {}

  $: nextLabel = currentStep == 1 ? "Avanti" : "Fine"
  $: nextAction = currentStep == 1 ? nextStep : openApartment

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

<div class="h-screen flex flex-col gap-4">
  <div class="flex-initial">
    <Heading customSize="text-3xl font-bold">
      Casa {$page.params.houseName} - Appartamento {$page.params.apartmentNumber}
    </Heading>
  </div>

  <div class="flex-auto">
    {#if guests.length == 0}
      <div class="flex flex-col gap-2">
        <StepIndicator {currentStep} {steps} />

        <!-- content belonging to step -->
        <div>
          {#if currentStep == 1}
            <!-- <Datepicker on:click={() => console.log("ifewofwo")} /> -->
          {:else}
            <Button>2</Button>
          {/if}
        </div>
      </div>
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
          <LeftArrow clazz="mr-2 w-5 h-5" />
          Torna a scelta periodo
        </PaginationItem>
      {/if}
      <PaginationItem class="flex items-center" on:click={nextAction}>
        {nextLabel}
        <RightArrow clazz="ml-2 w-5 h-5" />
      </PaginationItem>
    </div>
  </div>
</div>
