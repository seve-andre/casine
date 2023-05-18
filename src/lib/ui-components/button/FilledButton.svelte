<script lang="ts">
  import { Button } from "flowbite-svelte"
  import type { ButtonType } from "flowbite-svelte/dist/types"

  export let href = ""
  export let type: ButtonType = "button"
</script>

<Button {href} {type} on:click pill btnClass="btn filled-btn">
  <slot />
</Button>

<style>
  :global(.filled-btn) {
    background-color: var(--md-sys-color-primary);
    color: var(--md-sys-color-on-primary);
  }

  :global(.filled-btn:hover) {
    box-shadow: 0px 1px 2px var(--md-sys-color-shadow);
    transition: 0.3s;
  }
</style>
