<script lang="ts">
  import "~/lib/utils/DateUtils"
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte"
  import type { Rent } from "~/models/Rent"
  import type { Guest } from "~/models/Guest"

  export let rent: Rent
  export let guests: Guest[]

  const guestsInfo = ["Nome", "Cognome", "Data di nascita"]
  // guests need to be mapped with only the values needed by the table
  const guestsMapped = guests.map(guest => ({
    id: guest.id,
    firstName: guest.first_name,
    lastName: guest.last_name,
    birthDate: new Date(guest.birth_date),
  }))
</script>

<Table striped>
  <caption class="p-5 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
    Famiglia {"{nome}"}
    <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">
      La famiglia {"{nome}"} soggiornerà in questo appartamento dal {rent.start_date.toItalianFormat()} al {rent.end_date.toItalianFormat()}
    </p>
  </caption>
  <TableHead>
    {#each guestsInfo as info}
      <TableHeadCell>{info}</TableHeadCell>
    {/each}
  </TableHead>
  <TableBody>
    {#each guestsMapped as guest (guest.id)}
      <TableBodyRow>
        <TableBodyCell>{guest.firstName}</TableBodyCell>
        <TableBodyCell>{guest.lastName}</TableBodyCell>
        <TableBodyCell>{guest.birthDate.toItalianFormat()}</TableBodyCell>
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>
