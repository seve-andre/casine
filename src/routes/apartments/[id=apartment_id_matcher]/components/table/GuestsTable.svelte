<script lang="ts">
  import "~/lib/utils/date-utils"
  import {
    Button,
    Checkbox,
    Input,
    Label,
    Modal,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte"
  import type { Group, Guest, Rent } from "~/models"
  import FilledButton from "~/lib/ui-components/button/FilledButton.svelte"

  export let rent: Rent
  export let guests: Guest[]
  export let group: Group

  const guestsInfo = ["Nome", "Cognome", "Data di nascita"]
  // guests need to be mapped with only the values needed by the table
  const guestsMapped = guests.map(guest => ({
    id: guest.id,
    firstName: guest.first_name,
    lastName: guest.last_name,
    birthDate: new Date(guest.birth_date),
  }))

  let showNewGuestForm = false
</script>

<div class="table-container">
  <div class="table-wrapper">
    <Table striped divClass="table__main">
      <caption class="p-5 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
        Famiglia {group.nickname}
        <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">
          La famiglia {group.nickname} soggiornerà in questo appartamento dal {rent.start_date.toItalianFormat()} al {rent.end_date.toItalianFormat()}
        </p>
      </caption>
      <TableHead theadClass="text-sm bg-gray-100 dark:bg-gray-700">
        {#each guestsInfo as info}
          <TableHeadCell>{info}</TableHeadCell>
        {/each}
      </TableHead>
      <TableBody tableBodyClass="table__body">
        {#each guestsMapped as guest (guest.id)}
          <TableBodyRow>
            <TableBodyCell>{guest.firstName}</TableBodyCell>
            <TableBodyCell>{guest.lastName}</TableBodyCell>
            <TableBodyCell>{guest.birthDate.toItalianFormat()}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  <div class="table__controls">
    <FilledButton on:click={() => (showNewGuestForm = true)}>Aggiungi ospite</FilledButton>
  </div>

  <Modal bind:open={showNewGuestForm} size="xs" autoclose={false} class="w-full">
    <form class="flex flex-col space-y-3" action="#">
      <h1 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Aggiungi ospite</h1>
      <div class="grid gap-6 md:grid-cols-2">
        <div>
          <Label for="first-name" color="gray" class="block mb-2">Nome</Label>
          <Input id="first-name" value="" color="base" placeholder="Andrea" />
        </div>

        <div>
          <Label for="last-name" color="gray" class="block mb-2">Cognome</Label>
          <Input id="last-name" value="" color="base" placeholder="Severi" />
        </div>

        <div>
          <Label for="birth-date" color="gray" class="block mb-2">Data di nascita</Label>
          <Input type="date" id="birth-date" value="" color="base" />
        </div>
      </div>

      <FilledButton>Aggiungi</FilledButton>
    </form>
  </Modal>
</div>

<style>
  .table-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .table-wrapper {
    flex: 1 1 80%;
  }

  .table__controls {
    flex: 0 1 20%;
  }

  :global(.table__main) {
    height: 100%;

    /* border: 1px solid var(--md-sys-color-on-surface); */
    border-width: 1px;
    border-style: solid;
    /* dark: rgba(255, 255, 255, 0.12) */
    border-color: rgba(0, 0, 0, 0.12);
    border-radius: 5px;

    /* so that table corners don't get overlapped */
    padding: 0.25rem;
  }
</style>
