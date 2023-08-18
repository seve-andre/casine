<script lang="ts">
  import { Helper, Input, Label } from "flowbite-svelte"

  import type { GuestFormErrors } from "./guest-form-errors"

  export let firstName: string
  export let lastName: string
  export let birthdate: string

  export let errors: GuestFormErrors
</script>

<div class="grid gap-6 md:grid-cols-2">
  <div>
    <Label for="first-name" color={errors.onFirstName ? "red" : "gray"} class="block mb-2"
      >Nome</Label
    >
    <Input
      id="first-name"
      bind:value={firstName}
      color={errors.onFirstName ? "red" : "base"}
      placeholder="Andrea"
    />
    {#if errors.onFirstName}
      <Helper class="mt-2" color="red">{errors.onFirstName}</Helper>
    {/if}
  </div>

  <div>
    <Label for="last-name" color={errors.onLastName ? "red" : "gray"} class="block mb-2"
      >Cognome</Label
    >
    <Input
      id="last-name"
      bind:value={lastName}
      color={errors.onLastName ? "red" : "base"}
      placeholder="Severi"
    />
    {#if errors.onLastName}
      <Helper class="mt-2" color="red">{errors.onLastName}</Helper>
    {/if}
  </div>

  <div>
    <Label for="birth-date" color={errors.onBirthdate ? "red" : "gray"} class="block mb-2"
      >Data di nascita</Label
    >
    <Input
      type="date"
      id="birth-date"
      bind:value={birthdate}
      color={errors.onBirthdate ? "red" : "base"}
    />
    {#if errors.onBirthdate}
      <Helper class="mt-2" color="red">{errors.onBirthdate}</Helper>
    {/if}
  </div>
</div>
