<script lang="ts">
  import { Button } from "flowbite-svelte"

  export let disabled = false
  export let color: "blue" | "red" | "dark" = "blue"
</script>

<Button on:click pill outline {disabled} {color}>
  <slot />
</Button>
