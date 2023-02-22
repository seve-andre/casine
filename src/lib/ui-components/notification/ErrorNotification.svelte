<script lang="ts">
  import BaseNotification from "./BaseNotification.svelte"
  import { Cross2 } from "radix-icons-svelte"

  export let title = ""
</script>

<BaseNotification {title} icon={Cross2} color="red">
  <slot />
</BaseNotification>
