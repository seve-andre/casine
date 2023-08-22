<script lang="ts">
  import {
    Button,
    ButtonGroup,
    Heading,
    TableBody,
    TableBodyRow,
    TableHeadCell
  } from "flowbite-svelte"

  import { TableBodyCellMd, TableHeadMd, TableMd } from "$lib/ui-components"

  import type { PageData } from "./$types"
  import { getMonthNames } from "$lib/utils/date"
  import Check from "$lib/ui-components/icons/Check.svelte"

  export let data: PageData

  let selectedHouse: "A" | "B" = "A"

  const values = [
    {
      apartment: {
        id: 1,
        house_name: "A",
        apartment_number: 1
      },
      rent: {
        id: 1,
        apartment_id: 1,
        group_id: 1,
        start_date: "2023-04-01",
        end_date: "2023-04-30"
      },
      group: {
        id: 1,
        nickname: "Severi"
      }
    }
  ]

  const buttonClass = "hover:bg-primary-200 hover:text-gray-900 focus:text-gray-900"
  const selectedClass = `bg-primary-100 ${buttonClass}`
</script>

<div class="flex h-full flex-col gap-4 py-5 px-10">
  <div class="flex-initial">
    <Heading customSize="font-bold text-3xl">{data.title}</Heading>
  </div>

  <div class="flex-auto">
    <ButtonGroup>
      <Button
        class={selectedHouse === "A" ? selectedClass : buttonClass}
        on:click={() => (selectedHouse = "A")}
      >
        {#if selectedHouse === "A"}
          <Check class="mr-2 h-4 w-4" />
        {/if}
        Casa A
      </Button>

      <Button
        class={selectedHouse === "B" ? selectedClass : buttonClass}
        on:click={() => (selectedHouse = "B")}
      >
        {#if selectedHouse === "B"}
          <Check class="mr-2 h-4 w-4" />
        {/if}
        Casa B
      </Button>
    </ButtonGroup>

    <TableMd>
      <TableHeadMd>
        <TableHeadCell scope="col">Appartamento</TableHeadCell>

        <!-- aprile 4 - settembre 9 -->
        {#each getMonthNames(4, 9) as month}
          <TableHeadCell scope="col">{month}</TableHeadCell>
        {/each}
      </TableHeadMd>
      <TableBody>
        <TableBodyRow>
          <TableHeadCell scope="row">1</TableHeadCell>
          <TableBodyCellMd>y</TableBodyCellMd>
          <TableBodyCellMd>y</TableBodyCellMd>
          <TableBodyCellMd>n</TableBodyCellMd>
          <TableBodyCellMd>a</TableBodyCellMd>
          <TableBodyCellMd>a</TableBodyCellMd>
          <TableBodyCellMd>a</TableBodyCellMd>
        </TableBodyRow>
      </TableBody>
    </TableMd>
  </div>
</div>
