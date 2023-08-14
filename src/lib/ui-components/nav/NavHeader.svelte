<script lang="ts">
  import { Navbar } from "flowbite-svelte"

  import IconButton from "../buttons/IconButton.svelte"
  import LeftArrow from "../icons/LeftArrow.svelte"
</script>

<header class={$$props.class}>
  <Navbar navDivClass="mx-1 flex flex-wrap justify-between items-center">
    <IconButton href="/">
      <LeftArrow class="w-4 h-4" />
    </IconButton>
  </Navbar>
</header>
