<script lang="ts">
  import { Button, Heading, Label, Select } from "flowbite-svelte"
  import FilledButton from "../ui-components/button/FilledButton.svelte"

  let selectedApartment = 1

  let apartmentNumbers = [
    { value: "1", name: "App. 1" },
    { value: "2", name: "App. 2" },
    { value: "3", name: "App. 3" },
  ]
</script>

<div class="bg-gray-200 p-5 rounded-lg h-full flex flex-col">
  <div class="flex-initial">
    <Heading customSize="text-xl font-bold">Aggiungi ospite</Heading>
    <Heading customSize="text-base font-medium">Casa A</Heading>
    <Label>
      Scegli appartamento
      <Select
        class="mt-2"
        items={apartmentNumbers}
        bind:value={selectedApartment}
        placeholder="Seleziona uno degli appartamenti"
        on:change={() => console.log(`selectedApartment is ${selectedApartment}`)}
      />
    </Label>
  </div>

  <div class="flex-auto">
    <Button>agagagaga</Button>
  </div>

  <div class="flex-initial flex justify-end gap-5">
    <Button pill outline color="red">reset</Button>
    <FilledButton>invia</FilledButton>
  </div>
</div>
