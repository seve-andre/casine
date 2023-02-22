<script lang="ts">
  import { Image, Notification, type Component, type SvelteUIColor } from "@svelteuidev/core"

  export let title = ""
  export let icon: Component | HTMLOrSVGElement | undefined = undefined
  export let color: SvelteUIColor = "blue"

  let visible = true

  const close = () => {
    visible = false
  }

  setTimeout(close, 5000)
</script>

{#if visible}
  <Notification {title} {icon} {color} on:close={close} closeButtonProps={{ title: "Nascondi notifica" }}>
    <slot />
  </Notification>
{/if}
