<script lang="ts">
  import OutlinedButton from "./OutlinedButton.svelte"

  export let href: string | undefined = undefined
  export let type: "button" | "reset" = "button"
  export let size: "xs" | "sm" | "md" | "lg" | "xl" = "md"
  // export let color = "black"
</script>

<OutlinedButton on:click {href} {type} {size} class="bg-transparent text-black">
  <slot />
</OutlinedButton>

<style>
  :global(.icon-btn) {
    background-color: transparent;
    color: black;
  }
  :global(.icon-btn:hover) {
    background-color: rgb(103, 80, 164, 0.08);
  }
</style>
