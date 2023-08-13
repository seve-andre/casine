<script lang="ts">
  import OutlinedButton from "./OutlinedButton.svelte"

  export let href: string | undefined = undefined
  export let type: "button" | "reset" = "button"
  export let size: "xs" | "sm" | "md" | "lg" | "xl" = "md"
  export let color: "primary" | "red" = "primary"
</script>

<!-- a text button is an outlined button with transparent border -->
<OutlinedButton
  {href}
  {type}
  {size}
  {color}
  on:click
  on:change
  on:keydown
  on:keyup
  on:touchstart
  on:touchend
  on:touchcancel
  on:mouseenter
  on:mouseleave
  class="border-transparent"
>
  <slot />
</OutlinedButton>
