<script lang="ts">
  import { Button } from "@svelteuidev/core"

  export let disabled = false
  // export let icon = null
</script>

<Button variant="outline" {disabled}>
  <slot />
</Button>
