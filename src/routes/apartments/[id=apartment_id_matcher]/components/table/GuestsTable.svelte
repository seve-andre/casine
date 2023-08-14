<script lang="ts">
  import "~/lib/utils/date-utils"
  import { invoke } from "@tauri-apps/api/tauri"
  import { Modal, TableBody, TableBodyRow, TableHeadCell } from "flowbite-svelte"

  import GuestForm from "~/lib/page-components/common/GuestForm.svelte"
  import { FilledButton, TableBodyCellMd, TableHeadMd, TableMd } from "~/lib/ui-components"
  import type { Group, Guest, Rent } from "~/models"

  export let rent: Rent
  export let guests: Guest[]
  export let group: Group

  const guestsInfo = ["Nome", "Cognome", "Data di nascita"]
  let showNewGuestForm = false

  let firstName = ""
  let lastName = ""
  let birthDate = ""
</script>

<div class="h-full flex flex-col gap-4">
  <div class="flex-auto basis-4/5">
    <TableMd>
      <caption
        class="p-5 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800"
      >
        Famiglia {group.nickname}
        <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">
          La famiglia {group.nickname} soggiornerà in questo appartamento dal {rent.start_date.toItalianFormat()}
          al {rent.end_date.toItalianFormat()}
        </p>
      </caption>
      <TableHeadMd>
        {#each guestsInfo as info}
          <TableHeadCell>{info}</TableHeadCell>
        {/each}
      </TableHeadMd>
      <TableBody>
        {#each guests as guest (guest.id)}
          <TableBodyRow>
            <TableBodyCellMd>{guest.first_name}</TableBodyCellMd>
            <TableBodyCellMd>{guest.last_name}</TableBodyCellMd>
            <TableBodyCellMd>{guest.birth_date.toItalianFormat()}</TableBodyCellMd>
          </TableBodyRow>
        {/each}
      </TableBody>
    </TableMd>
  </div>

  <div class="flex-initial basis-1/5 flex justify-end items-center">
    <FilledButton on:click={() => (showNewGuestForm = true)}>Aggiungi ospite</FilledButton>
  </div>

  <Modal bind:open={showNewGuestForm} size="xs" autoclose={false} class="w-full">
    <h1 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Aggiungi ospite</h1>
    <GuestForm
      bind:firstName
      bind:lastName
      bind:birthDate
      onSubmitSuccess={() => {
        invoke("add_guest_to_group", {
          guest: {
            first_name: firstName,
            last_name: lastName,
            birth_date: birthDate
          },
          groupId: group.id,
          isLeader: false
        })

        showNewGuestForm = false
      }}
    />
  </Modal>
</div>
