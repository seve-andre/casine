<script lang="ts">
  import type { Guest } from "@/models/Guest"
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte"

  let tableHeaders = ["Nome", "Cognome", "Data di nascita"]
  let guests: Guest[] = [
    {
      id: 1,
      first_name: "Andrea",
      last_name: "Severi",
      birth_date: "02-10-2000",
    },
  ]
</script>

<Table striped>
  <TableHead>
    {#each tableHeaders as header}
      <TableHeadCell>{header}</TableHeadCell>
    {/each}
  </TableHead>
  <TableBody>
    {#each guests as guest (guest.id)}
      <TableBodyRow>
        <TableBodyCell>{guest.first_name}</TableBodyCell>
        <TableBodyCell>{guest.last_name}</TableBodyCell>
        <TableBodyCell>{guest.birth_date}</TableBodyCell>
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>
