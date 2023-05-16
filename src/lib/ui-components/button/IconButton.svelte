<script lang="ts">
  import { Button } from "flowbite-svelte"

  export let href = ""
</script>

<Button on:click {href} pill outline btnClass="btn text-btn">
  <slot />
</Button>

<!-- <Button href="/" pill outline>
  <LeftArrow class="w-4 h-4" />
</Button> -->

<style>
  :global(.text-btn) {
    background-color: transparent;
    color: var(--md-sys-color-on-surface);
  }

  :global(.text-btn:hover) {
    background-color: rgb(103, 80, 164, 0.08);
  }
</style>
