<script lang="ts">
  import BaseButton from "./BaseButton.svelte"

  export let href: string | undefined = undefined
  export let type: "button" | "submit" = "button"
  export let size: "xs" | "sm" | "md" | "lg" | "xl" = "md"
  export let color: "primary" | "red" = "primary"
</script>

<BaseButton
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
>
  <slot />
</BaseButton>
