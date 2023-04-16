<script lang="ts">
  import type { Rent } from "~/models/Rent"
  import type { Guest } from "~/models/Guest"
  import "~/lib/utils/DateUtils"
  import { Heading, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte"

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

<Heading customSize="text-xl font-semibold" tag="h2">
  Dal {rent.start_date.toItalianFormat()} al {rent.end_date.toItalianFormat()}
</Heading>
<Table striped>
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
