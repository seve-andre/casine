<script lang="ts">
  import { Button } from "flowbite-svelte"

  export let disabled = false
</script>

<Button on:click pill outline {disabled}>
  <slot />
</Button>
