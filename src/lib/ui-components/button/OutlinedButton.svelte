<script lang="ts">
  import { Button } from "flowbite-svelte"

  export let href = ""
</script>

<Button on:click {href} pill outline color="alternative">
  <slot />
</Button>
