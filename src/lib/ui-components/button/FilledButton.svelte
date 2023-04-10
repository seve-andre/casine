<script lang="ts">
  import { Button } from "flowbite-svelte"

  export let href = ""
  export let disabled = false
</script>

<Button {href} on:click pill {disabled}>
  <slot />
</Button>
