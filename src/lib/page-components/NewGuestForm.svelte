<script lang="ts">
  import { Button, Heading, Helper, Input, Label, Select } from "flowbite-svelte"
  import FilledButton from "@/lib/ui-components/button/FilledButton.svelte"
  import OutlinedButton from "@/lib/ui-components/button/OutlinedButton.svelte"

  let selectedApartment = 1
  let apartmentNumbers = Array.from({ length: 6 }, (_, i) => ({
    value: i + 1,
    name: `App. ${i + 1}`,
  }))

  // guest data
  let firstName: string | undefined = undefined
  let lastName: string | undefined = undefined
  let birthDate: string | undefined = undefined

  $: isFirstNameEmpty = firstName == undefined || firstName == ""
  $: isLastNameEmpty = lastName == undefined || lastName == ""
  $: isBirthDateEmpty = birthDate == undefined || birthDate == ""
  $: isFormValid = !isFirstNameEmpty && !isLastNameEmpty && !isBirthDateEmpty

  let firstNameTouched = false
  let lastNameTouched = false
  let birthDateTouched = false

  const resetForm = () => {
    firstName = undefined
    lastName = undefined
    birthDate = undefined
  }
</script>

<form class="bg-gray-200 p-5 rounded-lg h-full flex flex-col gap-5">
  <Heading customSize="text-xl font-bold">Aggiungi ospite</Heading>
  <div class="flex-initial">
    <Label>
      Scegli appartamento casa A
      <Select
        class="mt-2"
        items={apartmentNumbers}
        bind:value={selectedApartment}
        placeholder="Seleziona uno degli appartamenti"
      />
    </Label>
  </div>

  <div class="flex-auto">
    <div class="grid gap-6 md:grid-cols-2">
      <div>
        <Label for="first-name" color={isFirstNameEmpty && firstNameTouched ? "red" : "gray"} class="block mb-2">
          Nome
        </Label>
        <Input
          id="first-name"
          bind:value={firstName}
          color={isFirstNameEmpty && firstNameTouched ? "red" : "base"}
          placeholder="Andrea"
          on:keydown={() => (firstNameTouched = true)}
        />
        {#if isFirstNameEmpty && firstNameTouched}
          <Helper class="mt-2" color="red">Il nome è obbligatorio</Helper>
        {/if}
      </div>

      <div>
        <Label for="last-name" color={isLastNameEmpty && lastNameTouched ? "red" : "gray"} class="block mb-2">
          Cognome
        </Label>
        <Input
          id="last-name"
          color={isLastNameEmpty && lastNameTouched ? "red" : "base"}
          placeholder="Severi"
          bind:value={lastName}
          on:keydown={() => (lastNameTouched = true)}
        />
        {#if isLastNameEmpty && lastNameTouched}
          <Helper class="mt-2" color="red">Il cognome è obbligatorio</Helper>
        {/if}
      </div>

      <div>
        <Label for="birth-date" color={isBirthDateEmpty && birthDateTouched ? "red" : "gray"} class="block mb-2">
          Data di nascita
        </Label>
        <Input
          type="date"
          id="birth-date"
          color={isBirthDateEmpty && birthDateTouched ? "red" : "base"}
          bind:value={birthDate}
          on:keydown={() => (birthDateTouched = true)}
        />
        {#if isBirthDateEmpty && birthDateTouched}
          <Helper class="mt-2" color="red">La data di nascita è obbligatoria</Helper>
        {/if}
      </div>
    </div>
  </div>

  <div class="flex-initial flex justify-end gap-5">
    <OutlinedButton
      color="red"
      disabled={isFirstNameEmpty && isLastNameEmpty && isBirthDateEmpty}
      on:click={() => resetForm()}
    >
      Reset
    </OutlinedButton>
    <FilledButton disabled={!isFormValid}>Aggiungi</FilledButton>
  </div>
</form>
