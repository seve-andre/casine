<script lang="ts">
  import GuestFormData from "$lib/page-components/common/GuestFormData.svelte"

  import type { SecondStepErrors } from "./second-step"

  // data
  export let firstName: string
  export let lastName: string
  export let birthDate: string

  // errors
  export let errors: SecondStepErrors
</script>

<GuestFormData bind:firstName bind:lastName bind:birthDate bind:errors />
