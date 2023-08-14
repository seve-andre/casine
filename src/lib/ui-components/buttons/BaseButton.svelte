<script lang="ts">
  import { Button } from "flowbite-svelte"
  import type { ButtonType, SizeType } from "flowbite-svelte/dist/types"
  import { getContext } from "svelte"

  const group: SizeType = getContext("group")

  export let href: string | undefined = undefined
  export let type: ButtonType = "button"
  export let size: "xs" | "sm" | "md" | "lg" | "xl" = group ? "sm" : "md"
  export let outline = false
  export let color: "primary" | "red" | "alternative" = group ? "alternative" : "primary"
</script>

<Button
  {href}
  {type}
  {size}
  {outline}
  {color}
  pill
  on:click
  on:change
  on:keydown
  on:keyup
  on:touchstart
  on:touchend
  on:touchcancel
  on:mouseenter
  on:mouseleave
  class={$$props.class}
>
  <slot />
</Button>
