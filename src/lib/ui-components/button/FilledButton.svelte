<script lang="ts">
  import { Button } from "flowbite-svelte"

  export let href = ""
</script>

<Button {href} on:click pill btnClass="btn filled-btn">
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
