<script lang="ts">
  import OutlinedButton from "./OutlinedButton.svelte"

  export let href: string | undefined = undefined
  export let type: "button" | "reset" = "button"
  export let size: "xs" | "sm" | "md" | "lg" | "xl" = "md"
</script>

<!-- TODO: change bg and fg color on hover -->
<OutlinedButton on:click {href} {type} {size} class="border-transparent text-black">
  <slot />
</OutlinedButton>
