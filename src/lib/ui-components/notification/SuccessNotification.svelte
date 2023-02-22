<script lang="ts">
  import BaseNotification from "./BaseNotification.svelte"
  import { Check } from "radix-icons-svelte"

  export let title = ""
</script>

<BaseNotification {title} icon={Check} color="teal">
  <slot />
</BaseNotification>
