<script lang="ts">
  import { calculate_age } from "~/lib/utils/DateUtils"
  import type { Guest } from "~/models/Guest"
  import { Heading, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte"

  let guestsInfo = ["Nome", "Cognome", "Data di nascita"]

  export let guests: Guest[]

  // guests need to be mapped with only the values needed by the table
  let guestsMapped = guests.map(guest => ({
    id: guest.id,
    firstName: guest.first_name,
    lastName: guest.last_name,
    birthDate: new Date(guest.birth_date),
  }))
</script>

<Heading customSize="text-xl font-semibold" tag="h2">Dal ... al ...</Heading>
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
        <TableBodyCell>{guest.birthDate.toLocaleDateString("it-IT")}</TableBodyCell>
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>
