<script lang="ts">
  import { calculate_age } from "@/lib/utils/DateUtils"
  import type { Guest } from "@/models/Guest"
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte"

  let guestsInfo = ["Nome", "Cognome", "Data di nascita"]

  // guests need to be mapped with only the values needed by the table
  let guests: Guest[] = [
    {
      id: 1,
      first_name: "Andrea",
      last_name: "Severi",
      birth_date: "2000-10-02",
    },
    {
      id: 2,
      first_name: "Chiara",
      last_name: "Leonelli",
      birth_date: "2002-07-30",
    },
  ]

  let guestsMapped = guests.map(guest => ({
    id: guest.id,
    firstName: guest.first_name,
    lastName: guest.last_name,
    birthDate: new Date(guest.birth_date),
  }))
</script>

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
